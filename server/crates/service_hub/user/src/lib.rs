//! 用户信息管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    email::EmailDao, phone::PhoneDao, user_base::UserBaseDao, user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    email::EmailService, phone::PhoneService, user_base::UserBaseService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    email::EmailController, phone::PhoneController, user_base::UserBaseController,
    user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    email::EmailRouter, phone::PhoneRouter, user_base::UserBaseRouter,
    user_role_rel::UserRoleRelRouter, UserRouter,
};
