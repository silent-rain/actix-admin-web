//! RBAC 鉴权
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::{
    AUTH_WHITE_LIST, HEADERS_OPEN_API_AUTHORIZATION, HEADERS_OPEN_API_PASSPHRASE,
};

use context::Context;
use entity::{perm_token, user::user_base};
use response::Response;
use service_hub::{inject::AInjectProvider, permission::TokenService, user::UserBaseService};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage, HttpRequest,
};
use casbin::{
    prelude::{DefaultModel, Enforcer, MemoryAdapter},
    Adapter, CoreApi,
};
use futures::Future;
use tracing::error;

const MODEL: &str = "
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && (r.obj == p.obj) && (r.act == p.act)
";

const POLICY: &str = "
p, alice, /users, GET
p, bob, /users/1/status, PUT
g, alice, admin
";

/// OpenApi接口鉴权
#[derive(Default)]
pub struct CasbinAuth {}

impl<S, B> Transform<S, ServiceRequest> for CasbinAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CasbinAuthService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CasbinAuthService {
            service: Rc::new(service),
        }))
    }
}

pub struct CasbinAuthService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CasbinAuthService<S>
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

        let _provider = match req.app_data::<Data<AInjectProvider>>() {
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
            let path = req.path();

            // 白名单放行
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = service.call(req).await?;
                return Ok(resp);
            }

            // 获取上下文
            let ctx_resp = req.extensions().get::<Context>().cloned();
            let user_id = match ctx_resp {
                Some(ctx) => ctx.get_user_id(),
                None => {
                    let resp = service.call(req).await?;
                    return Ok(resp);
                }
            };
            // 获取权限标识

            // 获取权限信息

            // 加载模型
            let m = DefaultModel::from_str(MODEL).await.unwrap();
            // 加载策略
            let mut policy_model = DefaultModel::from_str(POLICY).await.unwrap();
            let mut a = MemoryAdapter::default();
            a.load_policy(&mut policy_model).await.unwrap();

            // 创建 Enforcer
            let mut e = Enforcer::new(m, a).await.unwrap();

            // 执行权限检查
            if e.enforce(("alice", "domain1", "data1", "read")).unwrap() {
                println!("权限允许");
            } else {
                println!("权限不允许");
            }

            // 响应
            let resp = service.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> CasbinAuthService<S> {
    /// 验证 OpenApi Token 及用户在状态
    ///
    /// TODO 登陆态后期可调整为缓存
    async fn verify_user_status(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<user_base::Model, code::ErrorMsg> {
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

#[cfg(test)]
mod test {
    use casbin::MgmtApi;

    use super::*;

    #[tokio::test]
    async fn test_csabin() {
        // 加载模型
        let m = DefaultModel::from_str(MODEL).await.unwrap();
        // 加载策略
        let adapter = MemoryAdapter::default();

        // 创建 Enforcer
        let mut e = Enforcer::new(m, adapter).await.unwrap();
        e.add_policy(
            ["admin", "/users", "GET"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
        .await
        .unwrap();
        e.add_policies(vec![["admin2", "/users/1/status", "PUT"]
            .iter()
            .map(|s| s.to_string())
            .collect()])
            .await
            .unwrap();

        // 添加角色赋值
        e.add_grouping_policies(vec![["alice", "admin"]
            .iter()
            .map(|s| s.to_string())
            .collect()])
            .await
            .unwrap();

        // 检查是否存在策略
        assert!(!e.has_policy(
            ["alice", "/users/1/status", "PUT"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));

        // 访问权限
        assert!(e.enforce(("alice", "/users", "GET")).unwrap());
        assert!(!e.enforce(("alice", "/users", "PUT")).unwrap());
        assert!(!e.enforce(("alice1", "/users", "PUT")).unwrap());
    }
}
