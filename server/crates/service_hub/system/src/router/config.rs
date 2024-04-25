//! 配置管理

use crate::ConfigController;

use actix_web::{web, Scope};

/// 路由器
pub struct ConfigRouter;

impl ConfigRouter {
    /// 注册`配置管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/configs")
            .route("", web::get().to(ConfigController::list))
            .route("/{id}", web::get().to(ConfigController::info))
            .route("", web::post().to(ConfigController::add))
            .route("/{id}", web::put().to(ConfigController::update))
            .route("/{id}/status", web::put().to(ConfigController::status))
            .route("/{id}", web::delete().to(ConfigController::delete))
    }
}
