//! 用户角色关联关系管理

use crate::{
    inject::AProvider,
    perm::{
        dto::user_role_rel::{AddUserRoleRelReq, GetUserRoleRelListReq},
        service::user_role_rel::UserRoleRelService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 获取用户角色关联列表
    pub async fn list(
        provider: Data<AProvider>,
        req: Query<GetUserRoleRelListReq>,
    ) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 创建用户角色关联
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserRoleRelReq>) -> impl Responder {
        let data = data.into_inner();
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.add(data).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 删除指定的用户角色关联关系
    pub async fn delete(provider: Data<AProvider>, user_id: Path<i32>) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.delete(*user_id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
