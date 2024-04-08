//! 角色管理

use crate::{
    inject::AProvider,
    perm::{
        dto::role::{AddRoleReq, GetRoleListReq, UpdateRoleReq},
        service::role::RoleService,
    },
};

use actix_validator::{Json, Query};
use context::Context;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct RoleController;

impl RoleController {
    /// 获取角色列表
    pub async fn list(provider: Data<AProvider>, req: Query<GetRoleListReq>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 获取角色信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 添加角色
    pub async fn add(
        ctx: Context,
        provider: Data<AProvider>,
        data: Json<AddRoleReq>,
    ) -> impl Responder {
        let user_id = ctx.get_user_id();
        let role_service: RoleService = provider.provide();
        let resp = role_service.add(user_id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 更新角色
    pub async fn update(
        ctx: Context,
        provider: Data<AProvider>,
        data: Json<UpdateRoleReq>,
    ) -> impl Responder {
        let user_id = ctx.get_user_id();
        let role_service: RoleService = provider.provide();
        let resp = role_service.update(user_id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 删除角色
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let role_service: RoleService = provider.provide();
        let resp = role_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
