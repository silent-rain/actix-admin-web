//! 路由层

pub mod api_operation;

use actix_web::{web, Scope};

/// 路由器
pub struct LogRouter;

impl LogRouter {
    /// 注册`日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/log")
            // 操作日志管理
            .service(api_operation::ApiOperationRouter::admin_register())
    }
}
