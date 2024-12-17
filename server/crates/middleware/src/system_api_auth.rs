//! 系统接口权限中间件
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{AUTH_WHITE_LIST, SYSTEM_API_AUTHORIZATION, SYSTEM_API_AUTHORIZATION_BEARER};

use entity::user::user_login_log;
use service_hub::{
    inject::AInjectProvider,
    log::UserLoginService,
    user::{cached::UserCached, dto::user_base::UserPermission, UserBaseService},
};

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

            // 不存在系统鉴权标识时, 则直接通过
            if req.headers().get(SYSTEM_API_AUTHORIZATION).is_none() {
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
            let (user_id, _) = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {:#?}", err);
                    return Err(Response::code(err).into());
                }
            };
            // 获取缓存
            if let Ok(permission) = UserCached::get_user_system_api_auth(user_id).await {
                // 设置上下文
                if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                    ctx.set_user_id(permission.user_id);
                    ctx.set_user_name(permission.username.clone());
                    ctx.set_api_auth_type(ApiAuthType::System);
                }
                info!(
                    "auth user req, cached, auth_type: {:?}, user_id: {}, username: {}",
                    ApiAuthType::System,
                    permission.user_id,
                    permission.username
                );
                let resp = service.call(req).await?;
                return Ok(resp);
            }

            // 验证登陆状态
            let user_login_id = match Self::verify_user_login(provider.clone(), system_token).await
            {
                Ok(v) => v,
                Err(err) => return Err(Response::err(err).into()),
            };
            // 获取用户权限
            let permission = match Self::user_permission(provider, user_id).await {
                Ok(v) => v,
                Err(err) => {
                    error!("获取权限失败, err: {:#?}", err);
                    return Err(Response::err(err).into());
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(permission.user_id);
                ctx.set_user_login_id(user_login_id);
                ctx.set_user_name(permission.username.clone());
                ctx.set_api_auth_type(ApiAuthType::System);
            }
            // 设置缓存
            UserCached::set_user_system_api_auth(user_id, permission.clone()).await;
            info!(
                "auth user req, auth_type: {:?}, user_id: {}, username: {}",
                ApiAuthType::System,
                permission.user_id,
                permission.username
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

    /// 获取用户权限
    async fn user_permission(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<UserPermission, code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service.get_sys_user_permission(user_id).await?;
        Ok(user)
    }

    /// 验证登陆状态
    /// TODO 后期可调整为缓存
    async fn verify_user_login(
        provider: AInjectProvider,
        token: String,
    ) -> Result<i32, code::ErrorMsg> {
        let user_login_service: UserLoginService = provider.provide();
        let user = user_login_service.info_by_token(token.clone()).await?;
        if user.status == user_login_log::enums::Status::Disabled as i8 {
            error!("user_id: {} token: {}, 当前登陆态已被禁用", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已被禁用, 请重新登陆"));
        }
        if user.status == user_login_log::enums::Status::Failed as i8 {
            error!("user_id: {} token: {}, 无效鉴权", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("无效鉴权, 请重新登陆"));
        }
        if user.status == user_login_log::enums::Status::Logout as i8 {
            error!("user_id: {} token: {}, 已登出", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("已登出, 请重新登陆"));
        }
        Ok(user.id)
    }
}
