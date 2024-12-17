//! 系统相关表

pub mod sys_config;
pub mod sys_dict_data;
pub mod sys_dict_dimension;
pub mod sys_image_captcha;
pub mod sys_image_resource;

pub use sys_config::Entity as SysConfig;
pub use sys_dict_data::Entity as SysDictData;
pub use sys_dict_dimension::Entity as SysDictDimension;
pub use sys_image_captcha::Entity as SysImageCaptcha;
pub use sys_image_resource::Entity as SysImageResource;
