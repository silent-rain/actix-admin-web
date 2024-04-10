//! 角色部门关系管理

use crate::perm::RoleDeptRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct RoleDeptRelRouter;

impl RoleDeptRelRouter {
    /// 注册`角色部门关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-role-rels")
            .route("", web::get().to(RoleDeptRelController::list))
            .route("/batch", web::post().to(RoleDeptRelController::batch_add))
            .route(
                "/batch",
                web::delete().to(RoleDeptRelController::batch_delete),
            )
    }
}
