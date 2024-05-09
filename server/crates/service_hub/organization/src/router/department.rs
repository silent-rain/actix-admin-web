//! 部门管理

use crate::controller::department::DepartmentController;

use actix_web::{web, Scope};

/// 路由器
pub struct DepartmentRouter;

impl DepartmentRouter {
    /// 注册`部门管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/departments")
            .route("", web::get().to(DepartmentController::list))
            .route("/tree", web::get().to(DepartmentController::tree))
            .route("/{id}", web::get().to(DepartmentController::info))
            .route("", web::post().to(DepartmentController::add))
            .route("/{id}", web::put().to(DepartmentController::update))
            .route("/{id}/status", web::put().to(DepartmentController::status))
            .route("/{id}", web::delete().to(DepartmentController::delete))
    }
}
