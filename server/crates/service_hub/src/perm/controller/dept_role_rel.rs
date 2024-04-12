//! 部门角色关系管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::dept_role_rel::{
            BatchAddDeptRoleRelReq, BatchDeleteDeptRoleRelReq, GetDeptRoleRelListReq,
        },
        service::dept_role_rel::DeptRoleRelService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct DeptRoleRelController;

impl DeptRoleRelController {
    /// 获取部门角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDeptRoleRelListReq>,
    ) -> impl Responder {
        let dept_role_rel_service: DeptRoleRelService = provider.provide();
        let resp = dept_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建部门角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddDeptRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let dept_role_rel_service: DeptRoleRelService = provider.provide();
        let resp = dept_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除部门角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteDeptRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let dept_role_rel_service: DeptRoleRelService = provider.provide();
        let resp = dept_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
