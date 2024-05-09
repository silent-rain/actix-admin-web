//! 登陆日志管理

use crate::controller::user_login::UserLoginController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserLoginRouter;

impl UserLoginRouter {
    /// 注册`登陆日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-logins")
            .route("", web::get().to(UserLoginController::list))
            .route("/{id}", web::get().to(UserLoginController::info))
            .route("/{id}", web::put().to(UserLoginController::update))
            .route("/{id}/status", web::put().to(UserLoginController::status))
    }
}
