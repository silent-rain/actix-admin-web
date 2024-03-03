//! 角色管理

use crate::{
    dto::perm::perm_role::{AddRoleReq, DeleteRoleReq, RoleInfoReq, RoleListReq, UserRoleListReq},
    inject::Provider,
    service::perm::perm_role::PermRoleService,
};

use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};
use actix_web_validator::{Json, Query};
use validator::Validate;

/// 控制器
pub struct Controller;

impl Controller {
    /// 获取所有角色列表
    pub async fn all(provider: Data<Provider>) -> impl Responder {
        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.all().await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 角色列表查询
    pub async fn list(provider: Data<Provider>, req: Query<RoleListReq>) -> impl Responder {
        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }

    /// 角色详情查询
    pub async fn info(provider: Data<Provider>, params: Query<RoleInfoReq>) -> impl Responder {
        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::build().code(Error::DbQueryEmptyError),
        };

        Response::build().data(result)
    }

    /// 添加角色信息
    pub async fn add(provider: Data<Provider>, data: Json<AddRoleReq>) -> impl Responder {
        let data = data.into_inner();
        if let Err(e) = data.validate() {
            return Response::build()
                .code(Error::InvalidParameterError)
                .msg(&e.to_string());
        }

        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }

    /// 删除角色
    pub async fn delete(provider: Data<Provider>, params: Query<DeleteRoleReq>) -> impl Responder {
        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().msg("删除成功")
    }
}

impl Controller {
    /// 通过用户ID获取角色列表
    pub async fn role_list(
        provider: Data<Provider>,
        req: Query<UserRoleListReq>,
    ) -> impl Responder {
        let perm_user_service: PermRoleService = provider.provide();
        let resp = perm_user_service.role_list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(results, total)
    }
}
