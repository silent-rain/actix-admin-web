//! 系统日志

use crate::controller::log::log_system::Controller;

use actix_web::{web, Scope};

/// 注册系统日志管理路由
pub fn register() -> Scope {
    web::scope("/log/system")
        .route("/list", web::get().to(Controller::list))
        .route("/info", web::get().to(Controller::info))
        .route("/add", web::post().to(Controller::add))
        .route("/delete", web::delete().to(Controller::delete))
}
