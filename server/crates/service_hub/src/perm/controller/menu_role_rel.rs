//! 菜单角色关系管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::menu_role_rel::{
            BatchAddMenuRoleRelReq, BatchDeleteMenuRoleRelReq, GetMenuRoleRelListReq,
        },
        service::menu_role_rel::MenuRoleRelService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct MenuRoleRelController;

impl MenuRoleRelController {
    /// 获取菜单角色关系列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetMenuRoleRelListReq>,
    ) -> impl Responder {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let resp = menu_role_rel_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 批量创建菜单角色关系
    pub async fn batch_add(
        provider: Data<AInjectProvider>,
        data: Json<BatchAddMenuRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let resp = menu_role_rel_service.batch_add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 批量删除菜单角色关系
    pub async fn batch_delete(
        provider: Data<AInjectProvider>,
        data: Json<BatchDeleteMenuRoleRelReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let resp = menu_role_rel_service.batch_delete(data.ids).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
