//! 权限管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    department::DepartmentDao, department_role_rel::DepartmentRoleRelDao, menu::MenuDao,
    menu_role_rel::MenuRoleRelDao, open_api::OpenApiDao, open_api_role_rel::OpenApiRoleRelDao,
    role::RoleDao, token::TokenDao, token_role_rel::TokenRoleRelDao, user::UserDao,
    user_email::UserEmailDao, user_phone::UserPhoneDao, user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    department::DepartmentService, department_role_rel::DepartmentRoleRelService,
    menu::MenuService, menu_role_rel::MenuRoleRelService, open_api::OpenApiService,
    open_api_role_rel::OpenApiRoleRelService, role::RoleService, token::TokenService,
    token_role_rel::TokenRoleRelService, user::UserService, user_email::UserEmailService,
    user_phone::UserPhoneService, user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    department::DepartmentController, department_role_rel::DepartmentRoleRelController,
    menu::MenuController, menu_role_rel::MenuRoleRelController, open_api::OpenApiController,
    open_api_role_rel::OpenApiRoleRelController, role::RoleController, token::TokenController,
    token_role_rel::TokenRoleRelController, user::UserController, user_email::UserEmailController,
    user_phone::UserPhoneController, user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    department::DepartmentRouter, department_role_rel::DepartmentRoleRelRouter, menu::MenuRouter,
    menu_role_rel::MenuRoleRelRouter, open_api::OpenApiRouter,
    open_api_role_rel::OpenApiRoleRelRouter, role::RoleRouter, token::TokenRouter,
    token_role_rel::TokenRoleRelRouter, user::UserRouter, user_email::UserEmailRouter,
    user_phone::UserPhoneRouter, user_role_rel::UserRoleRelRouter, PermissionRouter,
};
