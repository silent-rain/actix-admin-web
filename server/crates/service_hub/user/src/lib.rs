//! 用户信息管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    email::EmailDao, phone::PhoneDao, role::RoleDao, user_base::UserBaseDao,
    user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    email::EmailService, phone::PhoneService, role::RoleService, user_base::UserBaseService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    email::EmailController, phone::PhoneController, role::RoleController,
    user_base::UserBaseController, user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    email::EmailRouter, phone::PhoneRouter, role::RoleRouter, user_base::UserBaseRouter,
    user_role_rel::UserRoleRelRouter, UserRouter,
};
