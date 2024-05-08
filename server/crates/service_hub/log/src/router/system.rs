//! 系统日志

use crate::controller::system::SystemController;

use actix_web::{web, Scope};

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/systems")
            .route("", web::get().to(SystemController::list))
            .route("/{id}", web::get().to(SystemController::info))
        // .route("", web::post().to(LogSystemController::add))
        // .route("/{id}", web::delete().to(LogSystemController::delete))
    }
}
