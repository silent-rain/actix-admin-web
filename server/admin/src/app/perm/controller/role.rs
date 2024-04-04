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
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 获取所有角色列表
    pub async fn list(provider: Data<AProvider>, req: Query<RoleListReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 获取角色信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 添加角色
    pub async fn add(provider: Data<AProvider>, data: Json<AddRoleReq>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 删除角色
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: RoleService = provider.provide();
        let resp = perm_user_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
