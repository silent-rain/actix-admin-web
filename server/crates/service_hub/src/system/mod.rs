//! 系统管理
pub mod dto;
pub mod enums;

mod dao;
pub use dao::{
    captcha::CaptchaDao, config::ConfigDao, dict_data::DictDataDao, dict_dim::DictDimDao,
    icon::IconDao,
};

mod service;
pub use service::{
    captcha::CaptchaService, config::ConfigService, dict_data::DictDataService,
    dict_dim::DictDimService, icon::IconService,
};

mod controller;
pub use controller::{
    captcha::CaptchaController, config::ConfigController, dict_data::DictDataController,
    dict_dim::DictDimController, icon::IconController,
};

mod router;
pub use router::{
    captcha::CaptchaRouter, config::ConfigRouter, dict_data::DictDataRouter,
    dict_dim::DictDimRouter, icon::IconRouter, SystemRouter,
};
