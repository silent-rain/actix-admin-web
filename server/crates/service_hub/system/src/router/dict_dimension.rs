//! 字典维度管理

use crate::controller::dict_dimension::DictDimensionController;

use actix_web::{web, Scope};

/// 路由器
pub struct DictDimensionRouter;

impl DictDimensionRouter {
    /// 注册`字典维度管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/dict-dims")
            .route("", web::get().to(DictDimensionController::list))
            .route("/{id}", web::get().to(DictDimensionController::info))
            .route("", web::post().to(DictDimensionController::add))
            .route("/{id}", web::put().to(DictDimensionController::update))
            .route(
                "/{id}/status",
                web::put().to(DictDimensionController::status),
            )
            .route("/{id}", web::delete().to(DictDimensionController::delete))
    }
}
