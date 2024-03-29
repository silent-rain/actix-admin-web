//! 登陆

use crate::app::auth::LoginController;

use actix_web::{web, Scope};

/// 路由
pub struct LoginRouter;

impl LoginRouter {
    /// 注册登陆路由
    pub fn register() -> Scope {
        web::scope("/login").route("", web::post().to(LoginController::login))
    }
}
