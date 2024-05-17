//! 路由层
pub mod captcha;
pub mod login;
pub mod logout;
pub mod register;

use actix_web::{web, Scope};

/// 路由器
pub struct AuthRouter;

impl AuthRouter {
    /// 注册`认证管理`路由
    pub fn register() -> Scope {
        web::scope("/auth")
            // 生成验证码
            .service(captcha::GenCaptchaRouter::register())
            // 登陆
            .service(login::LoginRouter::register())
            // 登出
            .service(logout::LogoutRouter::register())
            // 注册用户
            .service(register::RegisterRouter::register())
    }
}
