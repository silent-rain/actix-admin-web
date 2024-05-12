//! 路由层

pub mod api_operation;
pub mod system;
pub mod user_login;
pub mod web_log;

use actix_web::{web, Scope};

/// 路由器
pub struct LogRouter;

impl LogRouter {
    /// 注册`日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/log")
            // 系统日志管理
            .service(system::SystemRouter::admin_register())
            // 登陆日志管理
            .service(user_login::UserLoginRouter::admin_register())
            // 操作日志管理
            .service(api_operation::ApiOperationRouter::admin_register())
            // WEB日志管理
            .service(web_log::WebLogRouter::admin_register())
    }
}
