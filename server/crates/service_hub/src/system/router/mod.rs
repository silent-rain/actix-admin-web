//! 路由层
pub mod captcha;
pub mod config;

use captcha::CaptchaRouter;
use config::ConfigRouter;

use actix_web::{web, Scope};

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/system")
            .service(CaptchaRouter::admin_register())
            .service(ConfigRouter::admin_register())
    }
}
