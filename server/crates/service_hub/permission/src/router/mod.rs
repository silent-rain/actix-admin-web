//! 路由层
pub mod department;
pub mod department_role_rel;
pub mod menu;
pub mod menu_role_rel;
pub mod openapi;
pub mod openapi_role_rel;
pub mod role;
pub mod token;
pub mod token_role_rel;
pub mod user;
pub mod user_email;
pub mod user_phone;
pub mod user_role_rel;

use actix_web::{web, Scope};

/// 路由器
pub struct PermissionRouter;

impl PermissionRouter {
    /// 注册`权限管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/permission")
            // 角色管理
            .service(role::RoleRouter::admin_register())
            // 用户管理
            .service(user::UserRouter::admin_register())
            // 用户手机号管理
            .service(user_phone::UserPhoneRouter::admin_register())
            // 用户邮箱管理
            .service(user_email::UserEmailRouter::admin_register())
            // 用户角色关系管理
            .service(user_role_rel::UserRoleRelRouter::admin_register())
            // 部门管理
            .service(department::DepartmentRouter::admin_register())
            // 部门角色关系管理
            .service(department_role_rel::DepartmentRoleRelRouter::admin_register())
            // 菜单管理
            .service(menu::MenuRouter::admin_register())
            // 菜单角色关系管理
            .service(menu_role_rel::MenuRoleRelRouter::admin_register())
            // 令牌管理
            .service(token::TokenRouter::admin_register())
            // 令牌角色关系管理
            .service(token_role_rel::TokenRoleRelRouter::admin_register())
            // OpenApi接口管理
            .service(openapi::OpenapiRouter::admin_register())
            // OpenApi接口角色关系管理
            .service(openapi_role_rel::OpenapiRoleRelRouter::admin_register())
    }
}
