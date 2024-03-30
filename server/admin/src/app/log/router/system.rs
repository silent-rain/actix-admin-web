//! 系统日志

use crate::app::log::LogSystemController;

use actix_web::{web, Scope};

/// 路由
pub struct SystemRouter;

impl SystemRouter {
    /// 注册系统日志管理路由
    pub fn register() -> Scope {
        web::scope("/log")
            // 系统日志管理
            .service(
                web::scope("/system")
                    .route("/list", web::get().to(LogSystemController::list))
                    .route("", web::get().to(LogSystemController::info))
                    .route("", web::post().to(LogSystemController::add))
                    .route("", web::delete().to(LogSystemController::delete)),
            )
    }
}
