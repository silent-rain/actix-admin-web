//! 部门角色关系管理

use crate::{
    dto::department_role_rel::{
        BatchAddDepartmentRoleRelReq, BatchDeleteDepartmentRoleRelReq, GetDepartmentRoleRelListReq,
    },
    service::department_role_rel::DepartmentRoleRelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct DepartmentRoleRelController;

impl DepartmentRoleRelController {
    /// 获取部门角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDepartmentRoleRelListReq>,
    ) -> impl Responder {
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let resp = department_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建部门角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddDepartmentRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let resp = department_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除部门角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteDepartmentRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let resp = department_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
