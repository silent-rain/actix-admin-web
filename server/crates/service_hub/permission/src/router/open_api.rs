//! OpenApi接口管理

use crate::OpenApiController;

use actix_web::{web, Scope};

/// 路由器
pub struct OpenApiRouter;

impl OpenApiRouter {
    /// 注册`OpenApi接口管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/openapi")
            .route("", web::get().to(OpenApiController::list))
            .route("/tree", web::get().to(OpenApiController::tree))
            .route("/{id}", web::get().to(OpenApiController::info))
            .route("", web::post().to(OpenApiController::add))
            .route("/{id}", web::put().to(OpenApiController::update))
            .route("/{id}/status", web::put().to(OpenApiController::status))
            .route("/{id}", web::delete().to(OpenApiController::delete))
    }
}
