//! 角色管理

use crate::perm::RoleController;

use actix_web::{web, Scope};

/// 路由器
pub struct RoleRouter;

impl RoleRouter {
    /// 注册`角色管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/roles")
            .route("", web::get().to(RoleController::list))
            .route("/{id}", web::get().to(RoleController::info))
            .route("", web::post().to(RoleController::add))
            .route("/update", web::put().to(RoleController::update))
            .route("/status", web::put().to(RoleController::status))
            .route("/{id}", web::delete().to(RoleController::delete))
    }
}
