//! 验证码

use crate::app::system::CaptchaController;

use actix_web::{web, Scope};

/// 路由
pub struct CaptchaRouter;

impl CaptchaRouter {
    /// 注册路由
    pub fn register() -> Scope {
        web::scope("/captcha")
            .route("/list", web::get().to(CaptchaController::list))
            .route("", web::get().to(CaptchaController::info))
            // .route("", web::post().to(CaptchaController::add))
            .route("", web::delete().to(CaptchaController::delete))
    }
}
