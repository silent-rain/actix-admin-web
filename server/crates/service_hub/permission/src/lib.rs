//! 权限管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    department::DepartmentDao, department_role_rel::DepartmentRoleRelDao, menu::MenuDao,
    menu_role_rel::MenuRoleRelDao, openapi::OpenapiDao, openapi_role_rel::OpenapiRoleRelDao,
    role::RoleDao, token::TokenDao, token_role_rel::TokenRoleRelDao, user::UserDao,
    user_email::UserEmailDao, user_phone::UserPhoneDao, user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    department::DepartmentService, department_role_rel::DepartmentRoleRelService,
    menu::MenuService, menu_role_rel::MenuRoleRelService, openapi::OpenapiService,
    openapi_role_rel::OpenapiRoleRelService, role::RoleService, token::TokenService,
    token_role_rel::TokenRoleRelService, user::UserService, user_email::UserEmailService,
    user_phone::UserPhoneService, user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    department::DepartmentController, department_role_rel::DepartmentRoleRelController,
    menu::MenuController, menu_role_rel::MenuRoleRelController, openapi::OpenapiController,
    openapi_role_rel::OpenapiRoleRelController, role::RoleController, token::TokenController,
    token_role_rel::TokenRoleRelController, user::UserController, user_email::UserEmailController,
    user_phone::UserPhoneController, user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    department::DepartmentRouter, department_role_rel::DepartmentRoleRelRouter, menu::MenuRouter,
    menu_role_rel::MenuRoleRelRouter, openapi::OpenapiRouter,
    openapi_role_rel::OpenapiRoleRelRouter, role::RoleRouter, token::TokenRouter,
    token_role_rel::TokenRoleRelRouter, user::UserRouter, user_email::UserEmailRouter,
    user_phone::UserPhoneRouter, user_role_rel::UserRoleRelRouter, PermissionRouter,
};
