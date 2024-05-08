//! 模板管理

use crate::controller::template::AppTemplateController;

use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

/// 路由器
pub struct AppTemplateRouter;

impl AppTemplateRouter {
    /// 注册路由
    pub fn admin_register() -> Scope {
        scope("/app-templates")
            .route("", get().to(AppTemplateController::list))
            .route("/{id}", get().to(AppTemplateController::info))
            .route("", post().to(AppTemplateController::add))
            .route("/{id}", put().to(AppTemplateController::update))
            .route("/{id}/status", put().to(AppTemplateController::status))
            .route("/batch", delete().to(AppTemplateController::batch_delete))
            .route("/{id}", delete().to(AppTemplateController::delete))
    }
}
