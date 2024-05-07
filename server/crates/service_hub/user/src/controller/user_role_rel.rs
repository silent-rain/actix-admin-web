//! 用户角色关系管理

use crate::{
    dto::user_role_rel::{
        BatchAddUserRoleRelReq, BatchDeleteUserRoleRelReq, GetUserRoleRelListReq,
    },
    service::user_role_rel::UserRoleRelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 获取用户角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserRoleRelListReq>,
    ) -> impl Responder {
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let resp = user_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建用户角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddUserRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let resp = user_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除用户角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteUserRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let resp = user_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
