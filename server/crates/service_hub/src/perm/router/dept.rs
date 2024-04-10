//! 部门管理

use crate::perm::DeptController;

use actix_web::{web, Scope};

/// 路由器
pub struct DeptRouter;

impl DeptRouter {
    /// 注册`部门管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/depts")
            .route("", web::get().to(DeptController::list))
            .route("/{id}", web::get().to(DeptController::info))
            .route("", web::post().to(DeptController::add))
            .route("/update", web::put().to(DeptController::update))
            .route("/status", web::put().to(DeptController::status))
            .route("/{id}", web::delete().to(DeptController::delete))
    }
}