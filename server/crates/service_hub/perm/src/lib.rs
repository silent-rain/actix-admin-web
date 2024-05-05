//! 权限管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    dept::DeptDao, dept_role_rel::DeptRoleRelDao, menu::MenuDao, menu_role_rel::MenuRoleRelDao,
    open_api::OpenApiDao, open_api_role_rel::OpenApiRoleRelDao, role::RoleDao, user::UserDao,
    user_email::UserEmailDao, user_phone::UserPhoneDao, user_role_rel::UserRoleRelDao,
    user_token::UserTokenDao, user_token_role_rel::UserTokenRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    dept::DeptService, dept_role_rel::DeptRoleRelService, menu::MenuService,
    menu_role_rel::MenuRoleRelService, open_api::OpenApiService,
    open_api_role_rel::OpenApiRoleRelService, role::RoleService, user::UserService,
    user_email::UserEmailService, user_phone::UserPhoneService, user_role_rel::UserRoleRelService,
    user_token::UserTokenService, user_token_role_rel::UserTokenRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    dept::DeptController, dept_role_rel::DeptRoleRelController, menu::MenuController,
    menu_role_rel::MenuRoleRelController, open_api::OpenApiController,
    open_api_role_rel::OpenApiRoleRelController, role::RoleController, user::UserController,
    user_email::UserEmailController, user_phone::UserPhoneController,
    user_role_rel::UserRoleRelController, user_token::UserTokenController,
    user_token_role_rel::UserTokenRoleRelController,
};

pub(crate) mod router;
pub use router::{
    dept::DeptRouter, dept_role_rel::DeptRoleRelRouter, menu::MenuRouter,
    menu_role_rel::MenuRoleRelRouter, open_api::OpenApiRouter,
    open_api_role_rel::OpenApiRoleRelRouter, role::RoleRouter, user::UserRouter,
    user_email::UserEmailRouter, user_phone::UserPhoneRouter, user_role_rel::UserRoleRelRouter,
    user_token::UserTokenRouter, user_token_role_rel::UserTokenRoleRelRouter, PermissionRouter,
};
