//! 用户角色关联关系管理

use crate::controller::perm::perm_user_role_rel::Controller;

use actix_web::{web, Scope};

/// 注册用户角色关联关系管理路由
pub fn register() -> Scope {
    web::scope("/user_role_rel")
        .route("/list", web::get().to(Controller::list))
        .route("", web::post().to(Controller::add))
        .route("", web::delete().to(Controller::delete))
}
