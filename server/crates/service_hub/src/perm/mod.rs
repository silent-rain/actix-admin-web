//! 权限管理
pub mod dto;

mod dao;
pub use dao::{role::RoleDao, user::UserDao, role_user_rel::UserRoleRelDao};

mod service;
pub use service::{role::RoleService, user::UserService, user_role_rel::UserRoleRelService};

mod controller;
pub use controller::{
    role::RoleController, user::UserController, user_role_rel::UserRoleRelController,
};

mod router;
pub use router::{role::RoleRouter, user::UserRouter, user_role_rel::UserRoleRelRouter};
