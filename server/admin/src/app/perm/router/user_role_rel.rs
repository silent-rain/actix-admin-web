//! 用户角色关联关系管理

use crate::app::perm::UserRoleRelController;

use actix_web::{web, Scope};

/// 路由
pub struct UserRoleRelRouter;

impl UserRoleRelRouter {
    /// 注册用户角色关联关系管理路由
    pub fn register() -> Scope {
        web::scope("/user_role_rel")
            .route("/list", web::get().to(UserRoleRelController::list))
            .route("", web::post().to(UserRoleRelController::add))
            .route("", web::delete().to(UserRoleRelController::delete))
    }
}
