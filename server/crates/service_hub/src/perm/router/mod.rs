//! 路由层
pub mod dept;
pub mod dept_role_rel;
pub mod role;
pub mod user;
pub mod user_role_rel;

use dept::DeptRouter;
use dept_role_rel::DeptRoleRelRouter;
use role::RoleRouter;
use user::UserRouter;
use user_role_rel::UserRoleRelRouter;

use actix_web::{web, Scope};

/// 路由器
pub struct PermissionRouter;

impl PermissionRouter {
    /// 注册`权限管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/perm")
            // 角色管理
            .service(RoleRouter::admin_register())
            // 用户管理
            .service(UserRouter::admin_register())
            // 用户角色关系管理
            .service(UserRoleRelRouter::admin_register())
            // 部门管理
            .service(DeptRouter::admin_register())
            // 部门角色关系管理
            .service(DeptRoleRelRouter::admin_register())
    }
}
