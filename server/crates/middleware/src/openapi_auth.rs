//! OpenApi权限中间件
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{AUTH_WHITE_LIST, OPENAPI_AUTHORIZATION, OPENAPI_PASSPHRASE};

use context::{ApiAuthType, Context};
use response::Response;
use service_hub::{
    inject::AInjectProvider,
    user::{cached::UserCached, dto::user_base::UserPermission, UserBaseService},
};

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

            // 不存在Openapi鉴权标识时, 则直接通过
            if req.headers().get(OPENAPI_AUTHORIZATION).is_none() {
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
            // 获取缓存
            if let Ok(permission) =
                UserCached::get_user_openapi_api_auth(openapi_token.clone()).await
            {
                // 设置上下文
                if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                    ctx.set_user_id(permission.user_id);
                    ctx.set_user_name(permission.username.clone());
                    ctx.set_api_auth_type(ApiAuthType::Openapi);
                }
                info!(
                    "auth user req, cached, auth_type: {:?}, user_id: {}, username: {}",
                    ApiAuthType::Openapi,
                    permission.user_id,
                    permission.username
                );
                let resp = service.call(req).await?;
                return Ok(resp);
            }

            // 获取用户权限
            let permission =
                match Self::user_permission(provider.clone(), openapi_token.clone(), passphras)
                    .await
                {
                    Ok(v) => v,
                    Err(err) => {
                        error!("获取权限失败, err: {:#?}", err);
                        return Err(Response::err(err).into());
                    }
                };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(permission.user_id);
                ctx.set_user_name(permission.username.clone());
                ctx.set_api_auth_type(ApiAuthType::Openapi);
            }
            // 设置缓存
            UserCached::set_user_openapi_api_auth(openapi_token, permission.clone()).await;
            info!(
                "auth user req, auth_type: {:?}, user_id: {}, username: {}",
                ApiAuthType::Openapi,
                permission.user_id,
                permission.username
            );

            // 响应
            let resp = service.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> OpenApiAuthService<S> {
    /// 获取用户权限
    async fn user_permission(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<UserPermission, code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service
            .get_token_user_permission(openapi_token, passphrase)
            .await?;
        Ok(user)
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
