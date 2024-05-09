//! OpenApi接口角色关系管理

use crate::{
    dto::openapi_role_rel::{
        BatchAddOpenapiRoleRelReq, BatchDeleteOpenapiRoleRelReq, GetOpenapiRoleRelListReq,
    },
    service::openapi_role_rel::OpenapiRoleRelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct OpenapiRoleRelController;

impl OpenapiRoleRelController {
    /// 获取OpenApi接口角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetOpenapiRoleRelListReq>,
    ) -> impl Responder {
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let resp = openapi_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建OpenApi接口角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddOpenapiRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let resp = openapi_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除OpenApi接口角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteOpenapiRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let resp = openapi_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
