//! 角色管理

use crate::app::perm::RoleController;

use actix_web::{web, Scope};

/// 路由
pub struct RoleRouter;

impl RoleRouter {
    /// 注册角色管理路由
    pub fn register() -> Scope {
        web::scope("/role")
            .route("/all", web::get().to(RoleController::all))
            .route("/list", web::get().to(RoleController::list))
            .route("", web::get().to(RoleController::info))
            .route("", web::post().to(RoleController::add))
            .route("", web::delete().to(RoleController::delete))
            .route("/role_list", web::get().to(RoleController::role_list))
    }
}
