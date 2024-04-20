//! 路由层
pub mod dept;
pub mod dept_role_rel;
pub mod menu;
pub mod menu_role_rel;
pub mod open_api;
pub mod open_api_role_rel;
pub mod role;
pub mod user;
pub mod user_role_rel;
pub mod user_token;
pub mod user_token_role_rel;

use actix_web::{web, Scope};

/// 路由器
pub struct PermissionRouter;

impl PermissionRouter {
    /// 注册`权限管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/perm")
            // 角色管理
            .service(role::RoleRouter::admin_register())
            // 用户管理
            .service(user::UserRouter::admin_register())
            // 用户角色关系管理
            .service(user_role_rel::UserRoleRelRouter::admin_register())
            // 部门管理
            .service(dept::DeptRouter::admin_register())
            // 部门角色关系管理
            .service(dept_role_rel::DeptRoleRelRouter::admin_register())
            // 菜单管理
            .service(menu::MenuRouter::admin_register())
            // 菜单角色关系管理
            .service(menu_role_rel::MenuRoleRelRouter::admin_register())
            // 用户Token令牌管理
            .service(user_token::UserTokenRouter::admin_register())
            // 用户Token令牌与角色关系管理
            .service(user_token_role_rel::UserTokenRoleRelRouter::admin_register())
            // OpenApi接口管理
            .service(open_api::OpenApiRouter::admin_register())
            // OpenApi接口角色关系管理
            .service(open_api_role_rel::OpenApiRoleRelRouter::admin_register())
    }
}
