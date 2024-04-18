//! ICON图片

use crate::system::ImageController;

use actix_web::{web, Scope};

/// 路由器
pub struct ImageRouter;

impl ImageRouter {
    /// 注册`ICON图片管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/images")
            .route("", web::get().to(ImageController::list))
            .route("/{id}", web::get().to(ImageController::info))
            .route("", web::get().to(ImageController::add))
            .route("", web::put().to(ImageController::update))
            .route("/batch", web::delete().to(ImageController::batch_delete))
            .route("/{id}", web::delete().to(ImageController::delete))
    }
}
