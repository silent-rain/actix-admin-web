//! 用户角色关系管理

use crate::perm::UserRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserRoleRelRouter;

impl UserRoleRelRouter {
    /// 注册`用户角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-role-rels")
            .route("", web::get().to(UserRoleRelController::list))
            .route("/batch", web::post().to(UserRoleRelController::batch_add))
            .route(
                "/batch",
                web::delete().to(UserRoleRelController::batch_delete),
            )
    }
}
