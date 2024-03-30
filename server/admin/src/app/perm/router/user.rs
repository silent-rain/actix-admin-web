//! 用户管理

use crate::app::perm::UserController;

use actix_web::{web, Scope};

/// 路由
pub struct UserRouter;

impl UserRouter {
    /// 注册用户管理路由
    pub fn register() -> Scope {
        web::scope("/user")
            .route("/list", web::get().to(UserController::list))
            .route("", web::get().to(UserController::info))
            .route("", web::post().to(UserController::add))
            .route("", web::delete().to(UserController::delete))
    }
}
