//! 权限管理
pub mod dto;

mod dao;
pub use dao::{
    dept::DeptDao, role::RoleDao, role_dept_rel::RoleDeptRelDao, role_user_rel::UserRoleRelDao,
    user::UserDao,
};

mod service;
pub use service::{
    dept::DeptService, role::RoleService, role_dept_rel::RoleDeptRelService, user::UserService,
    user_role_rel::UserRoleRelService,
};

mod controller;
pub use controller::{
    dept::DeptController, role::RoleController, role_dept_rel::RoleDeptRelController,
    user::UserController, user_role_rel::UserRoleRelController,
};

mod router;
pub use router::{
    dept::DeptRouter, role::RoleRouter, role_dept_rel::RoleDeptRelRouter, user::UserRouter,
    user_role_rel::UserRoleRelRouter,
};
