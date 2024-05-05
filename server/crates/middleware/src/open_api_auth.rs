//! OpenApi权限拦截器
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{
    AUTH_WHITE_LIST, HEADERS_AUTHORIZATION, HEADERS_OPEN_API_AUTHORIZATION,
    HEADERS_OPEN_API_PASSPHRASE,
};

use context::Context;
use entity::perm_user;
use response::Response;
use service_hub::{
    inject::AInjectProvider,
    permission::{
        enums::{UserStatus, UserTokenStatus},
        UserService, UserTokenService,
    },
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

        // 存在检查系统鉴权时, 则直接通过
        if req.headers().get(HEADERS_AUTHORIZATION).is_some() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let resp = fut.await?;
                Ok(resp)
            });
        }

        // 获取 Openapi 鉴权
        let (openapi_token, passphras) = match Self::get_openapi_token(inner_req.clone()) {
            Ok(v) => v,
            Err(err) => {
                return Box::pin(async move {
                    error!("获取鉴权标识失败, err: {:#?}", err);
                    Err(Response::err(err).with_msg("获取鉴权标识失败").into())
                })
            }
        };

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
            // 验证 OpenApi Token 及用户在状态
            let user = Self::verify_user_status(provider.clone(), openapi_token, passphras)
                .await
                .map_err(Response::err)?;

            // 添加上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(user.id);
                ctx.set_user_name(user.username.clone());
            }
            info!(
                "openapi auth user req, user_id: {}, user_name: {}",
                user.id, user.username
            );

            // 响应
            let resp = service.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> OpenApiAuthService<S> {
    /// 验证 OpenApi Token 及用户在状态
    ///
    /// TODO 登陆态后期可调整为缓存
    async fn verify_user_status(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<perm_user::Model, code::ErrorMsg> {
        let user_token_service: UserTokenService = provider.provide();
        let token = user_token_service
            .info_by_token(openapi_token.clone(), passphrase)
            .await?;
        if token.status == UserTokenStatus::Disabled as i8 {
            error!("user_id: {}, Token已被禁用", openapi_token.clone());
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("Token已被禁用"));
        }

        let user_service: UserService = provider.provide();
        let user = user_service.info(token.user_id).await?;
        if user.status == UserStatus::Disabled as i8 {
            error!("user_id: {}, 用户已被禁用", user.id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        Ok(user)
    }

    /// 获取OPEN API鉴权标识Token
    fn get_openapi_token(req: HttpRequest) -> Result<(String, String), code::ErrorMsg> {
        let token = req
            .headers()
            .get(HEADERS_OPEN_API_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if token.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }

        let passphras = match req.headers().get(HEADERS_OPEN_API_PASSPHRASE) {
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
