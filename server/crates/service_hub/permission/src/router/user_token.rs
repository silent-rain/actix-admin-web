//! 用户Token令牌管理

use crate::UserTokenController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserTokenRouter;

impl UserTokenRouter {
    /// 注册`用户Token令牌管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-tokens")
            .route("", web::get().to(UserTokenController::list))
            .route("/{id}", web::get().to(UserTokenController::info))
            .route("", web::post().to(UserTokenController::add))
            .route("/{id}", web::put().to(UserTokenController::update))
            .route("/{id}/status", web::put().to(UserTokenController::status))
            .route("/{id}", web::delete().to(UserTokenController::delete))
    }
}
