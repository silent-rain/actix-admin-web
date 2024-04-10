//! 角色与用户关系管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::user_role_rel::{
            BatchAddUserRoleRelReq, BatchDeleteUserRoleRelReq, GetUserRoleRelListReq,
        },
        service::user_role_rel::UserRoleRelService,
    },
};

use actix_validator::{Json, Query};
use context::Context;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 获取角色与用户关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserRoleRelListReq>,
    ) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 批量创建角色与用户关系
    pub async fn batch_add(
        ctx: Context,
        provider: Data<AInjectProvider>,
        data: Json<BatchAddUserRoleRelReq>,
    ) -> impl Responder {
        let user_id = ctx.get_user_id();
        let data = data.into_inner();
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.batch_add(user_id, data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 批量删除角色与用户关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteUserRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
