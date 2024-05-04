//! 路由层

pub mod table;

use actix_web::{web, Scope};

/// 路由器
pub struct InitializeRouter;

impl InitializeRouter {
    /// 注册`初始化管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/initialize")
            // 操作日志管理
            .service(table::TableRouter::admin_register())
    }
}
