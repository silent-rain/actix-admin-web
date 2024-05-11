//! 会员等级管理

use crate::controller::member_level::MemberLevelController;

use actix_web::{web, Scope};

/// 路由器
pub struct MemberLevelRouter;

impl MemberLevelRouter {
    /// 注册`会员等级管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/member-levels")
            .route("", web::get().to(MemberLevelController::list))
            .route("/{id}", web::get().to(MemberLevelController::info))
            .route("", web::post().to(MemberLevelController::add))
            .route("/{id}", web::put().to(MemberLevelController::update))
            .route("/{id}/status", web::put().to(MemberLevelController::status))
            .route("/{id}", web::delete().to(MemberLevelController::delete))
    }
}
