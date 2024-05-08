//! 用户邮箱管理

use crate::EmailController;

use actix_web::{web, Scope};

/// 路由器
pub struct EmailRouter;

impl EmailRouter {
    /// 注册`用户邮箱管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/emails")
            .route("", web::get().to(EmailController::list))
            .route("/{id}", web::get().to(EmailController::info))
            .route("", web::post().to(EmailController::add))
            .route("/{id}", web::put().to(EmailController::update))
            .route("/{id}", web::delete().to(EmailController::delete))
    }
}
