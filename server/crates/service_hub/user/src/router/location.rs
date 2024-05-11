//! 用户地理位置管理

use crate::controller::location::LocationController;

use actix_web::{web, Scope};

/// 路由器
pub struct LocationRouter;

impl LocationRouter {
    /// 注册`用户地理位置管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/locations")
            .route("", web::get().to(LocationController::list))
            .route("/{id}", web::get().to(LocationController::info))
            .route("", web::post().to(LocationController::add))
            .route("/{id}", web::put().to(LocationController::update))
            .route("/{id}", web::delete().to(LocationController::delete))
    }
}
