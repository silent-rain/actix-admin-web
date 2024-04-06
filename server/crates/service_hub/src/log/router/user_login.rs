//! 登陆日志

use crate::log::UserLoginController;

use actix_web::{web, Scope};

/// 路由
pub struct UserLoginRouter;

impl UserLoginRouter {
    /// 注册`登陆日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-logins")
            .route("", web::get().to(UserLoginController::list))
            .route("/{id}", web::get().to(UserLoginController::info))
            .route("/{id}/status", web::put().to(UserLoginController::status))
    }
}
