//! 模板管理

use crate::app::template::AppTemplateController;

use actix_web::{web, Scope};

/// 路由
pub struct AppTemplateRouter;

impl AppTemplateRouter {
    /// 注册路由
    pub fn register() -> Scope {
        web::scope("/appTemplate")
            .route("/all", web::get().to(AppTemplateController::all))
            .route("/list", web::get().to(AppTemplateController::list))
            .route("", web::get().to(AppTemplateController::info))
            .route("", web::post().to(AppTemplateController::add))
            .route(
                "/updateStatus",
                web::put().to(AppTemplateController::update_status),
            )
            .route("", web::delete().to(AppTemplateController::delete))
            .route(
                "/batchDelete",
                web::delete().to(AppTemplateController::batch_delete),
            )
    }
}
