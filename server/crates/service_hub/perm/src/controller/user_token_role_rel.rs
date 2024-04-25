//! 用户Token令牌与角色关系管理

use crate::{
    dto::user_token_role_rel::{
        BatchAddUserTokenRoleRelReq, BatchDeleteUserTokenRoleRelReq, GetUserTokenRoleRelListReq,
    },
    service::user_token_role_rel::UserTokenRoleRelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct UserTokenRoleRelController;

impl UserTokenRoleRelController {
    /// 获取用户令牌角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserTokenRoleRelListReq>,
    ) -> impl Responder {
        let dept_role_rel_service: UserTokenRoleRelService = provider.provide();
        let resp = dept_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建用户令牌角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddUserTokenRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let dept_role_rel_service: UserTokenRoleRelService = provider.provide();
        let resp = dept_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除用户令牌角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteUserTokenRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let dept_role_rel_service: UserTokenRoleRelService = provider.provide();
        let resp = dept_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
