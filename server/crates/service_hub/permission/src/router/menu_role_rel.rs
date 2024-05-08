//! 菜单角色关系管理

use crate::controller::menu_role_rel::MenuRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct MenuRoleRelRouter;

impl MenuRoleRelRouter {
    /// 注册`菜单角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/menu-role-rels")
            .route("", web::get().to(MenuRoleRelController::list))
            .route("/batch", web::post().to(MenuRoleRelController::batch_add))
            .route(
                "/batch",
                web::delete().to(MenuRoleRelController::batch_delete),
            )
    }
}
