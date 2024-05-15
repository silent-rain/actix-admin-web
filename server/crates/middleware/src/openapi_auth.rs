//! OpenApi权限中间件
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{
    AUTH_WHITE_LIST, OPENAPI_AUTHORIZATION, OPENAPI_PASSPHRASE, SYSTEM_API_AUTHORIZATION,
};

use context::{ApiAuthType, Context};
use entity::{perm_token, user::user_base};
use response::Response;
use service_hub::{inject::AInjectProvider, permission::TokenService, user::UserBaseService};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpRequest,
};
use futures::Future;
use tracing::{error, info};

/// OpenApi接口鉴权
#[derive(Default)]
pub struct OpenApiAuth {}

impl<S, B> Transform<S, ServiceRequest> for OpenApiAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OpenApiAuthService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OpenApiAuthService {
            service: Rc::new(service),
        }))
    }
}

pub struct OpenApiAuthService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for OpenApiAuthService<S>
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

            // 存在系统鉴权标识时, 则直接通过
            // TODO 带优化掉
            if req.headers().get(SYSTEM_API_AUTHORIZATION).is_some() {
                let resp = service.call(req).await?;
                return Ok(resp);
            }
            // 获取 Openapi 鉴权
            let (openapi_token, passphras) = match Self::get_openapi_token(inner_req.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("获取鉴权标识失败, err: {:#?}", err);
                    return Err(Response::err(err).into());
                }
            };
            // 验证 OpenApi Token 及用户
            let (user_id, username) = Self::verify_user(provider.clone(), openapi_token, passphras)
                .await
                .map_err(Response::err)?;

            // TODO 获取权限

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(user_id);
                ctx.set_user_name(username.clone());
                ctx.set_api_auth_type(ApiAuthType::Openapi);
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

impl<S> OpenApiAuthService<S> {
    /// 验证 OpenApi Token 及用户
    ///
    /// TODO 登陆态后期可调整为缓存
    async fn verify_user(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<(i32, String), code::ErrorMsg> {
        let token_service: TokenService = provider.provide();
        let token = token_service
            .info_by_token(openapi_token.clone(), passphrase)
            .await?;
        if token.status == perm_token::enums::Status::Disabled as i8 {
            error!("user_id: {}, Token已被禁用", openapi_token.clone());
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("Token已被禁用"));
        }

        let user_service: UserBaseService = provider.provide();
        let user = user_service.info(token.user_id).await?;
        if user.status == user_base::enums::Status::Disabled as i8 {
            error!("user_id: {}, 用户已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        Ok((user.id, user.username))
    }

    /// 获取OPEN API鉴权标识Token
    fn get_openapi_token(req: HttpRequest) -> Result<(String, String), code::ErrorMsg> {
        let token = req
            .headers()
            .get(OPENAPI_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if token.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }

        let passphras = match req.headers().get(OPENAPI_PASSPHRASE) {
            Some(v) => v.to_str().map_or("", |v| v),
            None => {
                return Err(code::Error::HeadersNotAuthorizationPassphrase
                    .into_msg()
                    .with_msg("鉴权口令不能为空"))
            }
        };

        Ok((token.to_string(), passphras.to_owned()))
    }
}
