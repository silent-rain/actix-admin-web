//! 路由层
pub mod captcha;
pub mod config;
pub mod icon;

use actix_web::{web, Scope};

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/system")
            .service(captcha::CaptchaRouter::admin_register())
            .service(config::ConfigRouter::admin_register())
            .service(icon::IconRouter::admin_register())
    }
}
