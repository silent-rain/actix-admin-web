//! 用户Token令牌管理

use crate::perm::DeptController;

use actix_web::{web, Scope};

/// 路由器
pub struct DeptRouter;

impl DeptRouter {
    /// 注册`用户Token令牌管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/depts")
            .route("", web::get().to(DeptController::list))
            .route("/tree", web::get().to(DeptController::tree))
            .route("/{id}", web::get().to(DeptController::info))
            .route("", web::post().to(DeptController::add))
            .route("/{id}", web::put().to(DeptController::update))
            .route("/{id}/status", web::put().to(DeptController::status))
            .route("/{id}", web::delete().to(DeptController::delete))
    }
}
