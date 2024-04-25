//! 路由层
pub mod captcha;
pub mod login;
pub mod register;

use captcha::GenCaptchaRouter;
use login::LoginRouter;
use register::RegisterRouter;

use actix_web::{web, Scope};

/// 路由器
pub struct AuthRouter;

impl AuthRouter {
    /// 注册`认证管理`路由
    pub fn register() -> Scope {
        web::scope("/auth")
            // 生成验证码
            .service(GenCaptchaRouter::register())
            // 登陆
            .service(LoginRouter::register())
            // 注册用户
            .service(RegisterRouter::register())
    }
}
