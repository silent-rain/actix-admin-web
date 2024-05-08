//! 用户手机号管理

use crate::PhoneController;

use actix_web::{web, Scope};

/// 路由器
pub struct PhoneRouter;

impl PhoneRouter {
    /// 注册`用户手机号管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/phones")
            .route("", web::get().to(PhoneController::list))
            .route("/{id}", web::get().to(PhoneController::info))
            .route("", web::post().to(PhoneController::add))
            .route("/{id}", web::put().to(PhoneController::update))
            .route("/{id}", web::delete().to(PhoneController::delete))
    }
}
