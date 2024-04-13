//! 系统管理
pub mod dto;
pub mod enums;

mod dao;
pub use dao::{captcha::CaptchaDao, config::ConfigDao, icon::IconDao};

mod service;
pub use service::{captcha::CaptchaService, config::ConfigService, icon::IconService};

mod controller;
pub use controller::{captcha::CaptchaController, config::ConfigController, icon::IconController};

mod router;
pub use router::{captcha::CaptchaRouter, config::ConfigRouter, icon::IconRouter, SystemRouter};
