//! OpenApi接口角色关系管理

use crate::perm::OpenApiRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct OpenApiRoleRelRouter;

impl OpenApiRoleRelRouter {
    /// 注册`OpenApi接口角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/openapi-role-rels")
            .route("", web::get().to(OpenApiRoleRelController::list))
            .route(
                "/batch",
                web::post().to(OpenApiRoleRelController::batch_add),
            )
            .route(
                "/batch",
                web::delete().to(OpenApiRoleRelController::batch_delete),
            )
    }
}
