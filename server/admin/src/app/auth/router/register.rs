//! 注册用户路由

use crate::app::auth::RegisterController;

use actix_web::{web, Scope};

/// 路由
pub struct RegisterRouter;

impl RegisterRouter {
    /// 注册用户路由
    pub fn register() -> Scope {
        web::scope("/register")
            .route("", web::post().to(RegisterController::register))
            .route(
                "/by-phone",
                web::post().to(RegisterController::phone_register),
            )
            .route(
                "/by-email",
                web::post().to(RegisterController::email_register),
            )
    }
}
