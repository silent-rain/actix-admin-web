//! 令牌角色关系管理

use crate::TokenRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct TokenRoleRelRouter;

impl TokenRoleRelRouter {
    /// 注册`令牌角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/token-role-rels")
            .route("", web::get().to(TokenRoleRelController::list))
            .route("/batch", web::post().to(TokenRoleRelController::batch_add))
            .route(
                "/batch",
                web::delete().to(TokenRoleRelController::batch_delete),
            )
    }
}
