//! OpenApi接口角色关系管理

use crate::controller::openapi_role_rel::OpenapiRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct OpenapiRoleRelRouter;

impl OpenapiRoleRelRouter {
    /// 注册`OpenApi接口角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/openapi-role-rels")
            .route("", web::get().to(OpenapiRoleRelController::list))
            .route(
                "/batch",
                web::post().to(OpenapiRoleRelController::batch_add),
            )
            .route(
                "/batch",
                web::delete().to(OpenapiRoleRelController::batch_delete),
            )
    }
}
