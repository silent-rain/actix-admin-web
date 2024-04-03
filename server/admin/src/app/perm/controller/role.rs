//! 角色管理

use crate::{
    app::perm::{
        dto::role::{AddRoleReq, RoleListReq},
        service::role::RoleService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct RoleController;

impl RoleController {
    /// 获取所有角色列表
    pub async fn all(provider: Data<AProvider>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.all().await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取所有角色列表
    pub async fn list(provider: Data<AProvider>, req: Query<RoleListReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取角色信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.info(*id).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 添加角色
    pub async fn add(provider: Data<AProvider>, data: Json<AddRoleReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.add(data.into_inner()).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 删除角色
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.delete(*id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
