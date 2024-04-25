//! 验证码

use system::CaptchaController;

use actix_web::{web, Scope};

/// 路由器
pub struct GenCaptchaRouter;

impl GenCaptchaRouter {
    /// 注册`生成验证码`路由
    pub fn register() -> Scope {
        web::scope("/captcha").route("", web::post().to(CaptchaController::add))
    }
}
