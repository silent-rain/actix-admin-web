//! 系统管理
pub mod constant;
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    captcha::CaptchaDao, config::ConfigDao, dict_data::DictDataDao, dict_dim::DictDimDao,
    image::ImageDao,
};

pub(crate) mod service;
pub use service::{
    captcha::CaptchaService, config::ConfigService, dict_data::DictDataService,
    dict_dim::DictDimService, image::ImageService,
};

pub(crate) mod controller;
pub use controller::{
    captcha::CaptchaController, config::ConfigController, dict_data::DictDataController,
    dict_dim::DictDimController, image::ImageController,
};

pub(crate) mod router;
pub use router::{
    captcha::CaptchaRouter, config::ConfigRouter, dict_data::DictDataRouter,
    dict_dim::DictDimRouter, image::ImageRouter, SystemRouter,
};
