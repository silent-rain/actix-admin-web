//! 公共服务
pub mod dao;
pub mod dto;
pub mod service;

pub(crate) mod controller;
pub use controller::{admin_web_site::AdminWebSiteController, health::HealthController};

pub(crate) mod router;
pub use router::{admin_web_site::AdminWebSiteRouter, health::HealthRouter};
