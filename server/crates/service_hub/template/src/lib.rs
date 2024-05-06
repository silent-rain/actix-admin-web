//! 模板管理
pub mod dto;

pub(crate) mod dao;
pub use dao::template::AppTemplateDao;

pub(crate) mod service;
pub use service::template::AppTemplateService;

pub(crate) mod controller;
pub use controller::template::AppTemplateController;

pub(crate) mod router;
pub use router::template::AppTemplateRouter;
