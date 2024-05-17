//! 登出

use crate::controller::logout::LogoutController;

use actix_web::{web, Scope};

/// 路由器
pub struct LogoutRouter;

impl LogoutRouter {
    /// 注册`用户登出`路由
    pub fn register() -> Scope {
        web::scope("/logout").route("", web::put().to(LogoutController::logout))
    }
}
