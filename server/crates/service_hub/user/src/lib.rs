//! 用户管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    user::UserDao, user_email::UserEmailDao, user_phone::UserPhoneDao,
    user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    user::UserService, user_email::UserEmailService, user_phone::UserPhoneService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    user::UserController, user_email::UserEmailController, user_phone::UserPhoneController,
    user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    user::ProfileRouter, user_email::UserEmailRouter, user_phone::UserPhoneRouter,
    user_role_rel::UserRoleRelRouter, UserRouter,
};
