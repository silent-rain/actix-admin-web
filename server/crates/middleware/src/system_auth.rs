//! 权限拦截器
use std::future::{ready, Ready};

use entity::{log_user_login, perm_user};
use service_hub::{inject::AInjectProvider, log::UserLoginService, user::UserService};

use crate::constant::{
    AUTH_WHITE_LIST, HEADERS_AUTHORIZATION, HEADERS_AUTHORIZATION_BEARER,
    HEADERS_OPEN_API_AUTHORIZATION,
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

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

/// 接口鉴权
#[derive(Default)]
pub struct SystemAuth {}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SystemAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SystemAuthService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SystemAuthService { service }))
    }
}

pub struct SystemAuthService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SystemAuthService<S>
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
        let path = req.path();
        // 白名单放行
        if AUTH_WHITE_LIST.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let resp = fut.await?;
                Ok(resp)
            });
        }

        let inner_req = req.request();

        // 存在 Openapi key 时, 则直接通过
        if req.headers().get(HEADERS_OPEN_API_AUTHORIZATION).is_some() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let resp = fut.await?;
                Ok(resp)
            });
        }

        // 获取系统鉴权标识Token
        let system_token = match Self::get_system_token(inner_req.clone()) {
            Ok(v) => v,
            Err(err) => {
                return Box::pin(async move {
                    error!("获取系统鉴权标识 Token 失败");
                    Err(Response::err(err).into())
                })
            }
        };
        // 检查系统鉴权
        let (user_id, user_name) = match Self::check_system_auth(system_token.clone()) {
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
        info!("system auth user req, user_id: {user_id}, user_name: {user_name}");

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

            // 验证当前登陆的用户是否被禁用
            if let Err(err) = Self::verify_user_login_disabled(provider, system_token).await {
                return Err(Response::err(err).into());
            }

            let resp = fut.await?;
            Ok(resp)
        })
    }
}

impl<S> SystemAuthService<S> {
    /// 检查系统鉴权
    fn check_system_auth(token: String) -> Result<(i32, String), code::Error> {
        // 解码 Token
        let claims =
            decode_token(&token).map_err(|err| code::Error::TokenDecode(err.to_string()))?;
        Ok((claims.user_id, claims.username))
    }

    /// 获取系统鉴权标识Token
    fn get_system_token(req: HttpRequest) -> Result<String, code::ErrorMsg> {
        let authorization = req
            .headers()
            .get(HEADERS_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }
        if !authorization.starts_with(HEADERS_AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {HEADERS_AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(code::Error::HeadersNotAuthorizationBearer
                .into_msg()
                .with_msg("非法请求"));
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
        if user.status == perm_user::enums::Status::Disabled as i8 {
            error!("user_id: {}, 用户已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        Ok(())
    }

    /// 验证当前登陆的用户是否被禁用
    /// TODO 后期可调整为缓存
    async fn verify_user_login_disabled(
        provider: AInjectProvider,
        token: String,
    ) -> Result<(), code::ErrorMsg> {
        let user_login_service: UserLoginService = provider.provide();
        let user = user_login_service.info_by_token(token.clone()).await?;
        if user.disabled == log_user_login::enums::DisabledStatus::Disabled as i8 {
            error!("user_id: {} token: {}, 当前登陆态已被禁用", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已被禁用, 请重新登陆"));
        }
        Ok(())
    }
}
