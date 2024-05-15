//! 系统接口权限中间件
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{
    AUTH_WHITE_LIST, OPENAPI_AUTHORIZATION, SYSTEM_API_AUTHORIZATION,
    SYSTEM_API_AUTHORIZATION_BEARER,
};

use entity::{log_user_login, user::user_base};
use service_hub::{inject::AInjectProvider, log::UserLoginService, user::UserBaseService};

use context::{ApiAuthType, Context};
use jwt::decode_token_with_verify;
use response::Response;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpRequest,
};
use futures::Future;
use tracing::{error, info};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.

/// 接口鉴权
#[derive(Default)]
pub struct SystemApiAuth {}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SystemApiAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SystemApiAuthService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SystemApiAuthService {
            service: Rc::new(service),
        }))
    }
}

pub struct SystemApiAuthService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SystemApiAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        let provider = match req.app_data::<Data<AInjectProvider>>() {
            Some(v) => v.as_ref().clone(),
            None => {
                return Box::pin(async move {
                    error!("获取服务实例失败");
                    Err(Response::code(code::Error::InjectAproviderObj).into())
                })
            }
        };
        Box::pin(async move {
            let inner_req = req.request();

            // 白名单放行
            let path = req.path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = service.call(req).await?;
                return Ok(resp);
            }

            // 存在 Openapi key 时, 则直接通过
            // TODO 带优化掉
            if req.headers().get(OPENAPI_AUTHORIZATION).is_some() {
                let resp = service.call(req).await?;
                return Ok(resp);
            }
            // 获取系统鉴权标识Token
            let system_token = match Self::get_system_api_token(inner_req) {
                Ok(v) => v,
                Err(err) => {
                    error!("获取系统鉴权标识 Token 失败, err: {:#?}", err);
                    return Err(Response::err(err).into());
                }
            };
            // 解析系统接口Token
            let (user_id, username) = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {:#?}", err);
                    return Err(Response::code(err).into());
                }
            };

            // 验证用户
            if let Err(err) = Self::verify_user(provider.clone(), user_id).await {
                return Err(Response::err(err).into());
            }
            // 验证登陆状态
            if let Err(err) = Self::verify_user_login(provider, system_token).await {
                return Err(Response::err(err).into());
            }

            // TODO 获取权限

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(user_id);
                ctx.set_user_name(username.clone());
                ctx.set_api_auth_type(ApiAuthType::System);
            }
            info!(
                "auth user req, auth_type: {:?}, user_id: {}, username: {}",
                ApiAuthType::Openapi,
                user_id,
                username
            );

            // 响应
            let resp = service.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> SystemApiAuthService<S> {
    /// 解析系统接口Token
    fn parse_system_token(token: String) -> Result<(i32, String), code::Error> {
        // 解码 Token
        let claims = decode_token_with_verify(&token)
            .map_err(|err| code::Error::TokenDecode(err.to_string()))?;
        Ok((claims.user_id, claims.username))
    }

    /// 获取系统接口鉴权Token
    fn get_system_api_token(req: &HttpRequest) -> Result<String, code::ErrorMsg> {
        let authorization = req
            .headers()
            .get(SYSTEM_API_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }
        if !authorization.starts_with(SYSTEM_API_AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {SYSTEM_API_AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(code::Error::HeadersNotAuthorizationBearer
                .into_msg()
                .with_msg("非法请求"));
        }

        let token = authorization.replace(SYSTEM_API_AUTHORIZATION_BEARER, "");

        Ok(token)
    }

    /// 验证用户
    async fn verify_user(provider: AInjectProvider, user_id: i32) -> Result<(), code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service.info(user_id).await?;
        if user.status == user_base::enums::Status::Disabled as i8 {
            error!("user_id: {}, 用户已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        Ok(())
    }

    /// 验证登陆状态
    /// TODO 后期可调整为缓存
    async fn verify_user_login(
        provider: AInjectProvider,
        token: String,
    ) -> Result<(), code::ErrorMsg> {
        let user_login_service: UserLoginService = provider.provide();
        let user = user_login_service.info_by_token(token.clone()).await?;
        if user.status == log_user_login::enums::Status::Disabled as i8 {
            error!("user_id: {} token: {}, 当前登陆态已被禁用", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已被禁用, 请重新登陆"));
        }
        if user.status == log_user_login::enums::Status::Failed as i8 {
            error!("user_id: {} token: {}, 无效鉴权", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("无效鉴权, 请重新登陆"));
        }
        Ok(())
    }
}
