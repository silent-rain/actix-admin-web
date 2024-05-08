//! 系统管理
pub mod constant;
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    config::ConfigDao, dict_data::DictDataDao, dict_dim::DictDimDao, image::ImageDao,
    image_captcha::ImageCaptchaDao,
};

pub(crate) mod service;
pub use service::{
    config::ConfigService, dict_data::DictDataService, dict_dim::DictDimService,
    image::ImageService, image_captcha::ImageCaptchaService,
};

pub(crate) mod controller;
pub use controller::{
    config::ConfigController, dict_data::DictDataController, dict_dim::DictDimController,
    image::ImageController, image_captcha::ImageCaptchaController,
};

pub(crate) mod router;
pub use router::{
    config::ConfigRouter, dict_data::DictDataRouter, dict_dim::DictDimRouter, image::ImageRouter,
    image_captcha::ImageCaptchaRouter, SystemRouter,
};
