//! 模板管理
pub mod dto;

mod dao;
pub use dao::template::AppTemplateDao;

mod service;
pub use service::template::AppTemplateService;

mod controller;
pub use controller::template::AppTemplateController;

mod router;
pub use router::template::AppTemplateRouter;
