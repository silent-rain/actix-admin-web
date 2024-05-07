//! 令牌角色关系管理

use crate::{
    dto::token_role_rel::{
        BatchAddTokenRoleRelReq, BatchDeleteTokenRoleRelReq, GetTokenRoleRelListReq,
    },
    service::token_role_rel::TokenRoleRelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct TokenRoleRelController;

impl TokenRoleRelController {
    /// 获取令牌角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetTokenRoleRelListReq>,
    ) -> impl Responder {
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let resp = token_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建令牌角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddTokenRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let resp = token_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除令牌角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteTokenRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let resp = token_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
