//! 角色管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::role::{AddRoleReq, GetRoleListReq, UpdateRoleReq, UpdateRoleStatusReq},
        service::role::RoleService,
    },
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
    /// 获取角色列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetRoleListReq>,
    ) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取角色信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加角色
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddRoleReq>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新角色
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateRoleReq>,
    ) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新角色状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateRoleStatusReq>,
    ) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.status(*id, data.clone().status as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除角色
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
