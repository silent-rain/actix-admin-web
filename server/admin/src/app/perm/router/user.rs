//! 用户管理

use crate::app::perm::UserController;

use actix_web::{web, Scope};

/// 路由
pub struct UserRouter;

impl UserRouter {
    /// 注册用户管理路由
    pub fn admin_register() -> Scope {
        web::scope("/users")
            .route("", web::get().to(UserController::list))
            .route("/{id}", web::get().to(UserController::info))
            // .route("/profile", web::get().to(UserController::profile)) // 获取用户个人信息
            .route("", web::post().to(UserController::add))
            .route("/{id}", web::delete().to(UserController::delete))
            .route("/{id}/roles", web::get().to(UserController::roles))
    }
}
