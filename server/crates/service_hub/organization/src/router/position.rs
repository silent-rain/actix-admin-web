//! 岗位管理

use crate::controller::position::PositionController;

use actix_web::{web, Scope};

/// 路由器
pub struct PositionRouter;

impl PositionRouter {
    /// 注册`岗位管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/positions")
            .route("", web::get().to(PositionController::list))
            .route("/{id}", web::get().to(PositionController::info))
            .route("", web::post().to(PositionController::add))
            .route("/{id}", web::put().to(PositionController::update))
            .route("/{id}/status", web::put().to(PositionController::status))
            .route("/{id}", web::delete().to(PositionController::delete))
    }
}
