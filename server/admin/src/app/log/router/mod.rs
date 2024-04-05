//! 路由层

pub mod system;
pub mod user_login;

use actix_web::{web, Scope};

/// 路由
pub struct LogRouter;

impl LogRouter {
    /// 注册`日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/log")
            // 系统日志管理
            .service(system::SystemRouter::admin_register())
            // 登陆日志管理
            .service(user_login::UserLoginRouter::admin_register())
    }
}
