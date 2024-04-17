//! 权限管理
pub mod dto;
pub mod enums;

mod dao;
pub use dao::{
    dept::DeptDao, dept_role_rel::DeptRoleRelDao, open_api::OpenApiDao,
    open_api_role_rel::OpenApiRoleRelDao, role::RoleDao, user::UserDao,
    user_role_rel::UserRoleRelDao,
};

mod service;
pub use service::{
    dept::DeptService, dept_role_rel::DeptRoleRelService, open_api::OpenApiService,
    open_api_role_rel::OpenApiRoleRelService, role::RoleService, user::UserService,
    user_role_rel::UserRoleRelService,
};

mod controller;
pub use controller::{
    dept::DeptController, dept_role_rel::DeptRoleRelController, open_api::OpenApiController,
    open_api_role_rel::OpenApiRoleRelController, role::RoleController, user::UserController,
    user_role_rel::UserRoleRelController,
};

mod router;
pub use router::{
    dept::DeptRouter, dept_role_rel::DeptRoleRelRouter, open_api::OpenApiRouter,
    open_api_role_rel::OpenApiRoleRelRouter, role::RoleRouter, user::UserRouter,
    user_role_rel::UserRoleRelRouter, PermissionRouter,
};
