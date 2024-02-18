//! 角色管理

use crate::controller::perm::perm_role::Controller;

use actix_web::{web, Scope};

/// 注册角色管理路由
pub fn register() -> Scope {
    web::scope("/role")
        .route("/all", web::get().to(Controller::all))
        .route("/list", web::get().to(Controller::list))
        .route("", web::get().to(Controller::info))
        .route("", web::post().to(Controller::add))
        .route("", web::delete().to(Controller::delete))
}
