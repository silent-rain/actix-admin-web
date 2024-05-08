//! 令牌管理

use crate::controller::token::TokenController;

use actix_web::{web, Scope};

/// 路由器
pub struct TokenRouter;

impl TokenRouter {
    /// 注册`令牌管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/tokens")
            .route("", web::get().to(TokenController::list))
            .route("/{id}", web::get().to(TokenController::info))
            .route("", web::post().to(TokenController::add))
            .route("/{id}", web::put().to(TokenController::update))
            .route("/{id}/status", web::put().to(TokenController::status))
            .route("/{id}", web::delete().to(TokenController::delete))
    }
}
