//! 字典维度管理

use crate::system::DictDimController;

use actix_web::{web, Scope};

/// 路由器
pub struct DictDimRouter;

impl DictDimRouter {
    /// 注册`字典维度管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/dict-dims")
            .route("", web::get().to(DictDimController::list))
            .route("/{id}", web::get().to(DictDimController::info))
            .route("", web::post().to(DictDimController::add))
            .route("/{id}", web::put().to(DictDimController::update))
            .route("/{id}/status", web::put().to(DictDimController::status))
            .route("/{id}", web::delete().to(DictDimController::delete))
    }
}
