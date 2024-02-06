//! 系统日志

use crate::controller::log::log_system::Controller;

use actix_web::{web, Scope};

/// 注册系统日志管理路由
pub fn register() -> Scope {
    web::scope("/log")
        // 系统日志管理
        .service(
            web::scope("/system")
                .route("/list", web::get().to(Controller::list))
                .route("", web::get().to(Controller::info))
                .route("", web::post().to(Controller::add))
                .route("", web::delete().to(Controller::delete)),
        )
}
