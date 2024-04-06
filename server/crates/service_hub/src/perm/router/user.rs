//! 用户管理

use crate::perm::UserController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserRouter;

impl UserRouter {
    /// 注册`用户管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/users")
            .route("/profile", web::get().to(UserController::profile))
            .route("", web::get().to(UserController::list))
            .route("/{id}", web::get().to(UserController::info))
            .route("", web::post().to(UserController::add))
            .route("/{id}", web::put().to(UserController::update))
            .route("/{id}", web::delete().to(UserController::delete))
            .route("/{id}/roles", web::get().to(UserController::roles))
    }
}
