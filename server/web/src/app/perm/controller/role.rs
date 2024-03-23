//! 角色管理

use crate::{
    app::perm::{
        dto::role::{AddRoleReq, DeleteRoleReq, RoleInfoReq, RoleListReq, UserRoleListReq},
        service::role::RoleService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use code::Error;
use response::Response;

use actix_web::{web::Data, Responder};
// use validator::Validate;

/// 控制器
pub struct RoleController;

impl RoleController {
    /// 获取所有角色列表
    pub async fn all(provider: Data<AProvider>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.all().await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 角色列表查询
    pub async fn list(provider: Data<AProvider>, req: Query<RoleListReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 角色详情查询
    pub async fn info(provider: Data<AProvider>, params: Query<RoleInfoReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.info(params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::code(Error::DbQueryEmptyError),
        };

        Response::ok().data(result)
    }

    /// 添加角色信息
    pub async fn add(provider: Data<AProvider>, data: Json<AddRoleReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.add(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }

    /// 删除角色
    pub async fn delete(provider: Data<AProvider>, params: Query<DeleteRoleReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().msg("删除成功")
    }
}

impl RoleController {
    /// 通过用户ID获取角色列表
    pub async fn role_list(
        provider: Data<AProvider>,
        req: Query<UserRoleListReq>,
    ) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.role_list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }
}
