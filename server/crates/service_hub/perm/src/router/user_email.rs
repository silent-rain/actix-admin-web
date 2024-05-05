//! 用户邮箱管理

use crate::UserEmailController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserEmailRouter;

impl UserEmailRouter {
    /// 注册`用户邮箱管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/roles")
            .route("", web::get().to(UserEmailController::list))
            .route("/{id}", web::get().to(UserEmailController::info))
            .route("", web::post().to(UserEmailController::add))
            .route("/{id}", web::put().to(UserEmailController::update))
            .route("/{id}", web::delete().to(UserEmailController::delete))
    }
}
