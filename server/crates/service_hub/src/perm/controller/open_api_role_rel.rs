//! OpenApi接口角色关系管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::open_api_role_rel::{
            BatchAddOpenApiRoleRelReq, BatchDeleteOpenApiRoleRelReq, GetOpenApiRoleRelListReq,
        },
        service::open_api_role_rel::OpenApiRoleRelService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct OpenApiRoleRelController;

impl OpenApiRoleRelController {
    /// 获取OpenApi接口角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetOpenApiRoleRelListReq>,
    ) -> impl Responder {
        let open_api_role_rel_service: OpenApiRoleRelService = provider.provide();
        let resp = open_api_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建OpenApi接口角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddOpenApiRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let open_api_role_rel_service: OpenApiRoleRelService = provider.provide();
        let resp = open_api_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除OpenApi接口角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteOpenApiRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let open_api_role_rel_service: OpenApiRoleRelService = provider.provide();
        let resp = open_api_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
