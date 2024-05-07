//! 用户信息管理

use crate::ProfileController;

use actix_web::{web, Scope};

/// 路由器
pub struct ProfileRouter;

impl ProfileRouter {
    /// 注册`用户信息管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/profiles")
            // .route("/profile", web::get().to(ProfileController::profile))
            .route("", web::get().to(ProfileController::list))
            .route("/{id}", web::get().to(ProfileController::info))
            .route("", web::post().to(ProfileController::add))
            .route("/{id}", web::put().to(ProfileController::update))
            .route("/{id}/status", web::put().to(ProfileController::status))
            .route("/{id}", web::delete().to(ProfileController::delete))
            .route("/{id}/roles", web::get().to(ProfileController::roles))
    }
}
