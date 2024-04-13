//! 系统管理
pub mod dto;
pub mod enums;

mod dao;
pub use dao::{captcha::CaptchaDao, config::ConfigDao};

mod service;
pub use service::{captcha::CaptchaService, config::ConfigService};

mod controller;
pub use controller::{captcha::CaptchaController, config::ConfigController};

mod router;
pub use router::{captcha::CaptchaRouter, config::ConfigRouter, SystemRouter};
