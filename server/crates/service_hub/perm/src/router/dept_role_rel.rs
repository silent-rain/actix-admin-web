//! 部门角色关系管理

use crate::DeptRoleRelController;

use actix_web::{web, Scope};

/// 路由器
pub struct DeptRoleRelRouter;

impl DeptRoleRelRouter {
    /// 注册`部门角色关系管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/dept-role-rels")
            .route("", web::get().to(DeptRoleRelController::list))
            .route("/batch", web::post().to(DeptRoleRelController::batch_add))
            .route(
                "/batch",
                web::delete().to(DeptRoleRelController::batch_delete),
            )
    }
}
