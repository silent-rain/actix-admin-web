//! 用户Token令牌与角色关系管理

use crate::UserTokenRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct UserTokenRoleRelRouter;

impl UserTokenRoleRelRouter {
    /// 注册`用户Token令牌与角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user-token-role-rels")
            .route("", web::get().to(UserTokenRoleRelController::list))
            .route(
                "/batch",
                web::post().to(UserTokenRoleRelController::batch_add),
            )
            .route(
                "/batch",
                web::delete().to(UserTokenRoleRelController::batch_delete),
            )
    }
}
