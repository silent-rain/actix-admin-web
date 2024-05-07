//! 用户手机号管理

use crate::UserPhoneController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserPhoneRouter;

impl UserPhoneRouter {
    /// 注册`用户手机号管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-phones")
            .route("", web::get().to(UserPhoneController::list))
            .route("/{id}", web::get().to(UserPhoneController::info))
            .route("", web::post().to(UserPhoneController::add))
            .route("/{id}", web::put().to(UserPhoneController::update))
            .route("/{id}", web::delete().to(UserPhoneController::delete))
    }
}
