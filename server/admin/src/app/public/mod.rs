//! 公共服务
pub mod dao;
pub mod dto;
pub mod service;

mod controller;
pub use controller::{health::HealthController, web_site::WebSiteController};

mod router;
pub use router::{health::HealthRouter, web_site::WebSiteRouter};
