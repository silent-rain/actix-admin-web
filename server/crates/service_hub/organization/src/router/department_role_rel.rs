//! 部门角色关系管理

use crate::DepartmentRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct DepartmentRoleRelRouter;

impl DepartmentRoleRelRouter {
    /// 注册`部门角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/department-role-rels")
            .route("", web::get().to(DepartmentRoleRelController::list))
            .route(
                "/batch",
                web::post().to(DepartmentRoleRelController::batch_add),
            )
            .route(
                "/batch",
                web::delete().to(DepartmentRoleRelController::batch_delete),
            )
    }
}
