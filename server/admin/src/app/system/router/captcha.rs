//! 验证码

use crate::app::system::CaptchaController;

use actix_web::{web, Scope};

/// 路由
pub struct CaptchaRouter;

impl CaptchaRouter {
    /// 注册验证码管理路由
    pub fn admin_register() -> Scope {
        web::scope("/captchas")
            .route("", web::get().to(CaptchaController::list))
            .route("/{id}", web::get().to(CaptchaController::info))
            .route("", web::get().to(CaptchaController::add))
            .route("/{id}", web::delete().to(CaptchaController::delete))
            .route("/batch", web::delete().to(CaptchaController::batch_delete))
    }
}
