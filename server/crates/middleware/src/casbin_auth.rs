//! RBAC 鉴权
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::constant::AUTH_WHITE_LIST;

use context::Context;
use response::Response;
use service_hub::permission::OpenapiService;
use service_hub::user::UserRoleRelService;
use service_hub::{inject::AInjectProvider, user::cached::UserCached};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use casbin::{
    prelude::{DefaultModel, Enforcer, MemoryAdapter},
    CoreApi, MgmtApi,
};
use futures::Future;
use tracing::{error, info};

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

const _POLICY: &str = "
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
            let path = req.match_info().as_str();
            let method = req.method().as_str();

            // 白名单放行
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = service.call(req).await?;
                return Ok(resp);
            }

            // 获取上下文
            let ctx_resp = req.extensions().get::<Context>().cloned();
            let user_id = match ctx_resp {
                Some(ctx) => {
                    // 判断是否已经鉴权, 如果没有则拒绝请求
                    if ctx.get_api_auth_type().is_none() {
                        error!("非法请求");
                        return Err(Response::code(code::Error::AuthIllegalRequest).into());
                    }

                    ctx.get_user_id()
                }
                None => {
                    let resp = service.call(req).await?;
                    return Ok(resp);
                }
            };

            // 获取缓存
            if let Ok(permission) = UserCached::get_user_openapi_access_permission(
                user_id,
                path.to_string(),
                method.to_string(),
            )
            .await
            {
                if permission {
                    info!(
                        "openapi access permission, cached, user_id: {user_id}, method: {method}, path: {path}"
                    );
                    let resp = service.call(req).await?;
                    return Ok(resp);
                }
            }

            // 获取接口角色关系列表
            let openapi_service: OpenapiService = provider.provide();
            let role_openapi_permissions = match openapi_service.role_openapi_permissions().await {
                Ok(v) => v,
                Err(err) => {
                    error!("{err:?}");
                    return Err(Response::err(err).into());
                }
            };
            // 获取用户角色关系列表
            let user_role_rel_service: UserRoleRelService = provider.provide();
            let user_role_rels = match user_role_rel_service.all().await {
                Ok((v, _)) => v,
                Err(err) => {
                    error!("{err:?}");
                    return Err(Response::err(err).into());
                }
            };

            // ["admin", "/users/1/status", "PUT"]
            let p_policies: Vec<Vec<String>> = role_openapi_permissions
                .iter()
                .map(|v| vec![v.role_id.to_string(), v.path.clone(), v.method.clone()])
                .collect();

            // ["alice", "admin"]
            let g_policies: Vec<Vec<String>> = user_role_rels
                .iter()
                .map(|v| vec![v.user_id.to_string(), v.role_id.to_string()])
                .collect();

            // 加载模型
            let m = DefaultModel::from_str(MODEL).await.unwrap();
            // 加载策略
            let adapter = MemoryAdapter::default();
            // 创建 Enforcer
            let mut e = Enforcer::new(m, adapter).await.unwrap();

            // 添加策略
            e.add_policies(p_policies).await.unwrap();
            // 添加角色
            e.add_grouping_policies(g_policies).await.unwrap();

            // 执行权限检查
            // ("alice", "/users", "GET")
            let result =
                match e.enforce((user_id.to_string(), path.to_string(), method.to_string())) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Casbin 策略执行失败, {err:?}");
                        return Err(Response::code(code::Error::CasbinEnforceError(
                            err.to_string(),
                        ))
                        .into());
                    }
                };
            if !result {
                error!("{user_id} {method} {path}, No access permission");
                return Err(Response::code(code::Error::CasbinNoAccessPermission).into());
            }

            // 设置缓存
            UserCached::set_user_openapi_access_permission(
                user_id,
                path.to_string(),
                method.to_string(),
            )
            .await;

            info!("openapi access permission, user_id: {user_id}, method: {method}, path: {path}");

            // 响应
            let resp = service.call(req).await?;
            Ok(resp)
        })
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
