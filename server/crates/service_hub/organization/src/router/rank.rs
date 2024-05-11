//! 职级管理

use crate::controller::rank::RankController;

use actix_web::{web, Scope};

/// 路由器
pub struct RankRouter;

impl RankRouter {
    /// 注册`职级管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/ranks")
            .route("", web::get().to(RankController::list))
            .route("/{id}", web::get().to(RankController::info))
            .route("", web::post().to(RankController::add))
            .route("/{id}", web::put().to(RankController::update))
            .route("/{id}/status", web::put().to(RankController::status))
            .route("/{id}", web::delete().to(RankController::delete))
    }
}
