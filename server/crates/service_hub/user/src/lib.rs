//! 用户信息管理
pub mod cached;
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    blockchain_wallet::BlockchainWalletDao, email::EmailDao, location::LocationDao,
    phone::PhoneDao, role::RoleDao, user_base::UserBaseDao, user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    blockchain_wallet::BlockchainWalletService, email::EmailService, location::LocationService,
    phone::PhoneService, role::RoleService, user_base::UserBaseService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    blockchain_wallet::BlockchainWalletController, email::EmailController,
    location::LocationController, phone::PhoneController, role::RoleController,
    user_base::UserBaseController, user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    blockchain_wallet::BlockchainWalletRouter, email::EmailRouter, location::LocationRouter,
    phone::PhoneRouter, role::RoleRouter, user_base::UserBaseRouter,
    user_role_rel::UserRoleRelRouter, UserRouter,
};
