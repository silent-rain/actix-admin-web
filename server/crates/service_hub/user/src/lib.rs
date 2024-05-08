//! 用户信息管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    email::EmailDao, user_base::UserBaseDao, user_phone::UserPhoneDao,
    user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    email::EmailService, user_base::UserBaseService, user_phone::UserPhoneService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    email::EmailController, user_base::UserBaseController, user_phone::UserPhoneController,
    user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    email::EmailRouter, user_base::UserBaseRouter, user_phone::UserPhoneRouter,
    user_role_rel::UserRoleRelRouter, UserRouter,
};
