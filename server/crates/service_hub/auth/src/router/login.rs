//! 登陆

use crate::LoginController;

use actix_web::{web, Scope};

/// 路由器
pub struct LoginRouter;

impl LoginRouter {
    /// 注册`用户登陆`路由
    pub fn register() -> Scope {
        web::scope("/login").route("", web::post().to(LoginController::login))
    }
}
