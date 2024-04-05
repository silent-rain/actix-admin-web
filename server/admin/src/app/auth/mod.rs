//! 鉴权
pub mod common;
pub mod dao;
pub mod dto;

mod service;
pub use service::{login::LoginService, register::RegisterService};

mod controller;
pub use controller::{login::LoginController, register::RegisterController};

mod router;
pub use router::{captcha::GenCaptchaRouter, login::LoginRouter, register::RegisterRouter};
