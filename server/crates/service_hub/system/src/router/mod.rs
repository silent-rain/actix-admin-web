//! 路由层
pub mod config;
pub mod dict_data;
pub mod dict_dimension;
pub mod image_captcha;
pub mod image_resource;

use actix_web::{web, Scope};

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/system")
            .service(image_captcha::ImageCaptchaRouter::admin_register())
            .service(image_resource::ImageResourceRouter::admin_register())
            .service(config::ConfigRouter::admin_register())
            .service(dict_dimension::DictDimensionRouter::admin_register())
            .service(dict_data::DictDataRouter::admin_register())
    }
}
