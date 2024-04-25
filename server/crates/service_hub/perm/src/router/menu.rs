//! 菜单管理

use crate::MenuController;

use actix_web::{web, Scope};

/// 路由器
pub struct MenuRouter;

impl MenuRouter {
    /// 注册`菜单管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/menus")
            .route("", web::get().to(MenuController::list))
            .route("/tree", web::get().to(MenuController::tree))
            .route("/{id}/children", web::get().to(MenuController::children))
            .route("/{id}", web::get().to(MenuController::info))
            .route("", web::post().to(MenuController::add))
            .route("/{id}", web::put().to(MenuController::update))
            .route("/{id}/status", web::put().to(MenuController::status))
            .route("/{id}", web::delete().to(MenuController::delete))
    }
}
