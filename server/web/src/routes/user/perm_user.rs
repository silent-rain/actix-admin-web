//! 用户管理

use crate::controller::user::perm_user::Controller;

use actix_web::{web, Scope};

/// 注册用户管理路由
pub fn register() -> Scope {
    web::scope("/user")
        .route("/list", web::get().to(Controller::list))
        .route("/info", web::get().to(Controller::info))
        .route("/add", web::post().to(Controller::add))
        .route("/delete", web::delete().to(Controller::delete))
}
