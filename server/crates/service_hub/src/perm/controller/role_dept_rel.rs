//! 角色部门关系管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::role_dept_rel::{
            BatchAddRoleDeptRelReq, BatchDeleteRoleDeptRelReq, GetRoleDeptRelListReq,
        },
        service::role_dept_rel::RoleDeptRelService,
    },
};

use actix_validator::{Json, Query};
use context::Context;
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct RoleDeptRelController;

impl RoleDeptRelController {
    /// 获取角色与部门关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetRoleDeptRelListReq>,
    ) -> impl Responder {
        let role_dept_rel_service: RoleDeptRelService = provider.provide();
        let resp = role_dept_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 批量创建角色与部门关系
    pub async fn batch_add(
        ctx: Context,
        provider: Data<AInjectProvider>,
        data: Json<BatchAddRoleDeptRelReq>,
    ) -> impl Responder {
        let user_id = ctx.get_user_id();
        let data = data.into_inner();
        let role_dept_rel_service: RoleDeptRelService = provider.provide();
        let resp = role_dept_rel_service.batch_add(user_id, data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 批量删除指定的角色与部门关系关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteRoleDeptRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let role_dept_rel_service: RoleDeptRelService = provider.provide();
        let resp = role_dept_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
