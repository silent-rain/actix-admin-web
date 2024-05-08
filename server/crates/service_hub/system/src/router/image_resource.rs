//! 图片资源管理

use crate::ImageResourceController;

use actix_web::{web, Scope};

/// 路由器
pub struct ImageResourceRouter;

impl ImageResourceRouter {
    /// 注册`图片资源管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/image-resources")
            .route("", web::get().to(ImageResourceController::list))
            .route("/{id}", web::get().to(ImageResourceController::info))
            .route(
                "/img/{hash}",
                web::get().to(ImageResourceController::info_by_hash),
            )
            .route(
                "/upload",
                web::post().to(ImageResourceController::upload_file),
            )
            .route(
                "/uploads",
                web::post().to(ImageResourceController::upload_files),
            )
            .route("/{id}", web::put().to(ImageResourceController::update))
            .route(
                "/batch",
                web::delete().to(ImageResourceController::batch_delete),
            )
            .route("/{id}", web::delete().to(ImageResourceController::delete))
    }
}
