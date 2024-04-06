//! 系统管理
pub mod dto;

mod dao;
pub use dao::captcha::CaptchaDao;

mod service;
pub use service::captcha::CaptchaService;

mod controller;
pub use controller::captcha::CaptchaController;

mod router;
pub use router::captcha::CaptchaRouter;
