//! 用户管理

use crate::controller::perm::perm_user::Controller;

use actix_web::{web, Scope};

/// 注册用户管理路由
pub fn register() -> Scope {
    web::scope("/user")
        .route("/list", web::get().to(Controller::list))
        .route("", web::get().to(Controller::info))
        .route("", web::post().to(Controller::add))
        .route("", web::delete().to(Controller::delete))
}
