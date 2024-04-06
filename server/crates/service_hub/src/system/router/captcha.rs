//! 验证码

use crate::system::CaptchaController;

use actix_web::{web, Scope};

/// 路由器
pub struct CaptchaRouter;

impl CaptchaRouter {
    /// 注册`验证码管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/captchas")
            .route("/batch", web::delete().to(CaptchaController::batch_delete))
            .route("", web::get().to(CaptchaController::list))
            .route("/{id}", web::get().to(CaptchaController::info))
            // .route("", web::get().to(CaptchaController::add))
            .route("/{id}", web::delete().to(CaptchaController::delete))
    }
}