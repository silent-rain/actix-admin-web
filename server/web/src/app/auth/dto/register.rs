//! 注册

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 注册手机用户
#[derive(Serialize, Deserialize, Validate)]
pub struct PhoneRegisterReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub username: String,
    pub gender: i8,
    #[validate(range(min = 18, max = 22, message = "Age must be between 18 to 22"))]
    pub age: i32,
    pub birthday: Option<String>,
    pub phone: String,
    #[validate(
        email,
        contains(pattern = "gmail", message = "Email must be valid gmail address")
    )]
    pub email: Option<String>,
    pub password: String,
    pub avatar: Option<String>,
    pub captcha_id: String,
    pub captcha: String,
}

/// 注册邮件用户
#[derive(Serialize, Deserialize, Validate)]
pub struct EmailRegisterReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub username: String,
    pub gender: i8,
    #[validate(range(min = 18, max = 22, message = "Age must be between 18 to 22"))]
    pub age: i32,
    pub birthday: Option<String>,
    pub phone: String,
    #[validate(
        email,
        contains(pattern = "gmail", message = "Email must be valid gmail address")
    )]
    pub email: Option<String>,
    pub password: String,
    pub avatar: Option<String>,
    pub captcha_id: String,
    pub captcha: String,
}
