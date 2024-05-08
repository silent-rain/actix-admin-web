//! 图片验证码管理

use crate::ImageCaptchaController;

use actix_web::{web, Scope};

/// 路由器
pub struct ImageCaptchaRouter;

impl ImageCaptchaRouter {
    /// 注册`图片验证码管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/image-captchas")
            .route("", web::get().to(ImageCaptchaController::list))
            .route("/{id}", web::get().to(ImageCaptchaController::info))
            // .route("", web::get().to(ImageCaptchaController::add))
            .route(
                "/batch",
                web::delete().to(ImageCaptchaController::batch_delete),
            )
            .route("/{id}", web::delete().to(ImageCaptchaController::delete))
    }
}
