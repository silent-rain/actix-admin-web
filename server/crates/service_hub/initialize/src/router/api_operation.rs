//! API操作日志

use crate::ApiOperationController;

use actix_web::{web, Scope};

/// 路由器
pub struct ApiOperationRouter;

impl ApiOperationRouter {
    /// 注册`API操作日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/api-operations")
            .route("", web::get().to(ApiOperationController::list))
            .route("/{id}", web::get().to(ApiOperationController::info))
        // .route("", web::post().to(ApiOperationController::add))
        // .route("/{id}", web::delete().to(ApiOperationController::delete))
    }
}
