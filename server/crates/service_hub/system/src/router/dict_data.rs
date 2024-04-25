//! 字典数据管理

use crate::DictDataController;

use actix_web::{web, Scope};

/// 路由器
pub struct DictDataRouter;

impl DictDataRouter {
    /// 注册`字典数据管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/dict-datas")
            .route("", web::get().to(DictDataController::list))
            .route("/{id}", web::get().to(DictDataController::info))
            .route("", web::post().to(DictDataController::add))
            .route("/{id}", web::put().to(DictDataController::update))
            .route("/{id}/status", web::put().to(DictDataController::status))
            .route("/{id}", web::delete().to(DictDataController::delete))
    }
}
