//! WEB日志管理

use crate::controller::web_log::WebLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct WebLogRouter;

impl WebLogRouter {
    /// 注册`WEB日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/webs")
            .route("", web::get().to(WebLogController::list))
            .route("/{id}", web::get().to(WebLogController::info))
            .route("", web::post().to(WebLogController::add))
    }
}
