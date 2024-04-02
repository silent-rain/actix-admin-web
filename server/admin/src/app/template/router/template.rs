//! 模板管理

use crate::app::template::AppTemplateController;

use actix_web::{web, Scope};

/// 路由
pub struct AppTemplateRouter;

impl AppTemplateRouter {
    /// 注册路由
    pub fn register() -> Scope {
        web::scope("/app-templates")
            // TODO all=True?
            // .route("/all", web::get().to(AppTemplateController::all))
            .route("", web::get().to(AppTemplateController::list))
            .route("/{id}", web::get().to(AppTemplateController::info))
            .route("", web::post().to(AppTemplateController::add))
            .route("", web::put().to(AppTemplateController::update))
            .route("/{id}/status", web::put().to(AppTemplateController::status))
            .route("/{id}", web::delete().to(AppTemplateController::delete))
            .route(
                "/{id}/batch",
                web::delete().to(AppTemplateController::batch_delete),
            )
    }
}