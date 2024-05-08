//! OpenApi接口管理

use crate::controller::openapi::OpenapiController;

use actix_web::{web, Scope};

/// 路由器
pub struct OpenapiRouter;

impl OpenapiRouter {
    /// 注册`OpenApi接口管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/openapi")
            .route("", web::get().to(OpenapiController::list))
            .route("/tree", web::get().to(OpenapiController::tree))
            .route("/{id}", web::get().to(OpenapiController::info))
            .route("", web::post().to(OpenapiController::add))
            .route("/{id}", web::put().to(OpenapiController::update))
            .route("/{id}/status", web::put().to(OpenapiController::status))
            .route("/{id}", web::delete().to(OpenapiController::delete))
    }
}
