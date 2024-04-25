//! 鉴权
pub mod common;
pub mod dao;
pub mod dto;
pub mod enums;

pub(crate) mod service;
pub use service::{login::LoginService, register::RegisterService};

pub(crate) mod controller;
pub use controller::{login::LoginController, register::RegisterController};

pub(crate) mod router;
pub use router::{
    captcha::GenCaptchaRouter, login::LoginRouter, register::RegisterRouter, AuthRouter,
};
