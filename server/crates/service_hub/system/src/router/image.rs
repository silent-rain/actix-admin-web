//! 图片

use crate::ImageController;

use actix_web::{web, Scope};

/// 路由器
pub struct ImageRouter;

impl ImageRouter {
    /// 注册`图片管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/images")
            .route("", web::get().to(ImageController::list))
            .route("/{id}", web::get().to(ImageController::info))
            .route("/img/{hash}", web::get().to(ImageController::info_by_hash))
            .route("/upload", web::post().to(ImageController::upload_file))
            .route("/uploads", web::post().to(ImageController::upload_files))
            .route("/{id}", web::put().to(ImageController::update))
            .route("/batch", web::delete().to(ImageController::batch_delete))
            .route("/{id}", web::delete().to(ImageController::delete))
    }
}