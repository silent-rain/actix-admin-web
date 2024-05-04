//! 服务初始化管理
pub mod asset;

pub mod dto;
pub mod enums;

pub(crate) mod dao;

pub use dao::table::TableDao;

pub(crate) mod service;
pub use service::table::TableService;

pub(crate) mod controller;
pub use controller::table::TableController;

pub(crate) mod router;
pub use router::{table::TableRouter, InitializeRouter};
