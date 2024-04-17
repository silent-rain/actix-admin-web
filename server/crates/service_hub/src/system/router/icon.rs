//! ICON图片

use crate::system::IconController;

use actix_web::{web, Scope};

/// 路由器
pub struct IconRouter;

impl IconRouter {
    /// 注册`ICON图片管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/icons")
            .route("", web::get().to(IconController::list))
            .route("/{id}", web::get().to(IconController::info))
            .route("", web::get().to(IconController::add))
            .route("", web::put().to(IconController::update))
            .route("/batch", web::delete().to(IconController::batch_delete))
            .route("/{id}", web::delete().to(IconController::delete))
    }
}
