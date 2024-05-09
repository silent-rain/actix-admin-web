//! 权限管理
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    menu::MenuDao, menu_role_rel::MenuRoleRelDao, openapi::OpenapiDao,
    openapi_role_rel::OpenapiRoleRelDao, token::TokenDao, token_role_rel::TokenRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    menu::MenuService, menu_role_rel::MenuRoleRelService, openapi::OpenapiService,
    openapi_role_rel::OpenapiRoleRelService, token::TokenService,
    token_role_rel::TokenRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    menu::MenuController, menu_role_rel::MenuRoleRelController, openapi::OpenapiController,
    openapi_role_rel::OpenapiRoleRelController, token::TokenController,
    token_role_rel::TokenRoleRelController,
};

pub(crate) mod router;
pub use router::{
    menu::MenuRouter, menu_role_rel::MenuRoleRelRouter, openapi::OpenapiRouter,
    openapi_role_rel::OpenapiRoleRelRouter, token::TokenRouter, token_role_rel::TokenRoleRelRouter,
    PermissionRouter,
};
