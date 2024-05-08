//! 路由层
pub mod template;

use actix_web::{web, Scope};

/// 路由器
pub struct TemplateRouter;

impl TemplateRouter {
    /// 注册`权限管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/template").service(template::AppTemplateRouter::admin_register())
    }
}
