//! 权限拦截器
use std::future::{ready, Ready};

use service_hub::{
    auth::enums::{UserLoginStatus, UserStatus},
    inject::AInjectProvider,
    log::UserLoginService,
    perm::UserService,
};

use crate::constant::{
    HEADERS_AUTHORIZATION, HEADERS_AUTHORIZATION_BEARER, HEADERS_OPEN_API_AUTHORIZATION,
};

use context::Context;
use jwt::decode_token;
use response::Response;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpRequest,
};
use futures::future::LocalBoxFuture;
use tracing::{error, info};

/// 白名单
const WHITE_LIST: [&str; 4] = [
    "/api/v1/health",
    "/api/v1/captcha",
    "/api/v1/login",
    "/api/v1/register",
];

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

/// 接口鉴权
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let path = req.path();
        // 白名单放行
        if WHITE_LIST.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let resp = fut.await?;
                Ok(resp)
            });
        }

        let inner_req = req.request();

        let openapi_token = Self::get_openapi_token(inner_req.clone());
        // 存在 Openapi key 时, 则直接通过
        if openapi_token.is_ok() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let resp = fut.await?;
                Ok(resp)
            });
        }
        // TODO API 鉴权

        // 获取系统鉴权标识Token
        let system_token = match Self::get_system_token(inner_req.clone()) {
            Ok(v) => v,
            Err(err) => {
                return Box::pin(async move {
                    error!("获取系统鉴权标识 Token 失败");
                    Err(Response::code(err).into())
                })
            }
        };
        // 检查系统鉴权
        let (user_id, user_name) = match Self::check_system_auth(system_token) {
            Ok(v) => v,
            Err(err) => {
                return Box::pin(async move {
                    error!("检查系统鉴权异常");
                    Err(Response::code(err).into())
                })
            }
        };
        // 添加上下文
        if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
            ctx.set_user_id(user_id);
            ctx.set_user_name(user_name.clone());
        }
        info!("user req, user_id: {user_id}, user_name: {user_name}");

        let provider = match req.app_data::<Data<AInjectProvider>>() {
            Some(v) => v.as_ref().clone(),
            None => {
                return Box::pin(async move {
                    error!("获取服务实例失败");
                    Err(Response::code(code::Error::InjectAproviderObj).into())
                })
            }
        };

        // 响应
        let fut = self.service.call(req);
        let provider = provider.clone();
        Box::pin(async move {
            // 验证用户及用户在状态
            if let Err(err) = Self::verify_user_status(provider.clone(), user_id).await {
                return Err(Response::err(err).into());
            }

            // TODO 待完善
            // 验证当前登陆的用户是否被禁用
            if let Err(err) = Self::verify_user_login_status(provider, user_id).await {
                return Err(Response::err(err).into());
            }

            let resp = fut.await?;
            Ok(resp)
        })
    }
}

impl<S> AuthMiddleware<S> {
    /// 检查系统鉴权
    fn check_system_auth(token: String) -> Result<(i32, String), code::Error> {
        // 解码 Token
        let claims =
            decode_token(&token).map_err(|err| code::Error::TokenDecode(err.to_string()))?;
        Ok((claims.user_id, claims.username))
    }

    /// 获取系统鉴权标识Token
    fn get_system_token(req: HttpRequest) -> Result<String, code::Error> {
        let authorization = req
            .headers()
            .get(HEADERS_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("用户请求标识未空, 非法请求");
            return Err(code::Error::HeadersNotAuthorization);
        }
        if !authorization.starts_with(HEADERS_AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {HEADERS_AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(code::Error::HeadersNotAuthorizationBearer);
        }

        let token = authorization.replace(HEADERS_AUTHORIZATION_BEARER, "");

        Ok(token)
    }

    /// 验证用户及用户在状态
    async fn verify_user_status(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<(), code::ErrorMsg> {
        let user_service: UserService = provider.provide();
        let user = user_service.info(user_id).await?;
        if user.status == UserStatus::Disabled as i8 {
            error!("user_id: {}, 用户已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已失效, 请重新登陆"));
        }
        Ok(())
    }

    /// 验证当前登陆的用户是否被禁用
    async fn verify_user_login_status(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<(), code::ErrorMsg> {
        let user_login_service: UserLoginService = provider.provide();
        let user = user_login_service.info_by_user_id(user_id).await?;
        if user.status == UserLoginStatus::Failed as i8 {
            error!("user_id: {}, 当前登陆态已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已失效, 请重新登陆"));
        }
        Ok(())
    }

    /// 获取OPEN API鉴权标识Token
    fn get_openapi_token(req: HttpRequest) -> Result<String, code::ErrorMsg> {
        let open_api_authorization = req
            .headers()
            .get(HEADERS_OPEN_API_AUTHORIZATION)
            .map_or("default", |v| v.to_str().map_or("", |v| v));

        if !open_api_authorization.is_empty() {
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("非法请求"));
        }
        Ok(open_api_authorization.to_string())
    }
}
