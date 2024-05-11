//! 路由层
pub mod department;
pub mod department_role_rel;
pub mod position;
pub mod rank;

use actix_web::{web, Scope};

/// 路由器
pub struct OrganizationRouter;

impl OrganizationRouter {
    /// 注册`组织管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/organization")
            // 部门管理
            .service(department::DepartmentRouter::admin_register())
            // 部门角色关系管理
            .service(department_role_rel::DepartmentRoleRelRouter::admin_register())
            // 岗位管理
            .service(position::PositionRouter::admin_register())
            // 职级管理
            .service(rank::RankRouter::admin_register())
    }
}
