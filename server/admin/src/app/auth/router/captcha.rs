//! 验证码

use crate::app::system::CaptchaController;

use actix_web::{web, Scope};

/// 路由
pub struct GenCaptchaRouter;

impl GenCaptchaRouter {
    /// 注册`生成验证码`路由
    pub fn register() -> Scope {
        web::scope("/captcha").route("", web::post().to(CaptchaController::add))
    }
}
