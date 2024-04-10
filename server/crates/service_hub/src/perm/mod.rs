//! 权限管理
pub mod dto;

mod dao;
pub use dao::{
    dept::DeptDao, dept_role_rel::DeptRoleRelDao, role::RoleDao, user_role_rel::UserRoleRelDao,
    user::UserDao,
};

mod service;
pub use service::{
    dept::DeptService, dept_role_rel::DeptRoleRelService, role::RoleService, user::UserService,
    user_role_rel::UserRoleRelService,
};

mod controller;
pub use controller::{
    dept::DeptController, dept_role_rel::DeptRoleRelController, role::RoleController,
    user::UserController, user_role_rel::UserRoleRelController,
};

mod router;
pub use router::{
    dept::DeptRouter, dept_role_rel::DeptRoleRelRouter, role::RoleRouter, user::UserRouter,
    user_role_rel::UserRoleRelRouter,
};
