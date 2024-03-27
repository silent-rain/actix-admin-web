//! 注册

use std::str::FromStr;

use actix_validator::Validate;

use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

/// 注册类型
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RegisterType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "email")]
    Email,
}

impl Default for RegisterType {
    fn default() -> Self {
        Self::Phone
    }
}

/// 实现FromStr trait来定义如何从字符串解析为RegisterType
impl FromStr for RegisterType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "phone" => Ok(RegisterType::Phone),
            "email" => Ok(RegisterType::Email),
            _ => Err(()),
        }
    }
}

/// 注册用户
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegisterReq {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub register_type: RegisterType,
    pub username: String,
    pub gender: i8,
    pub age: i32,
    pub birthday: Option<String>,
    pub password: String,
    pub avatar: Option<String>,
    #[serde(default)]
    pub captcha_id: String,
    #[serde(default)]
    pub captcha: String,
}

/// 注册手机用户
#[derive(Serialize, Deserialize, Validate)]
pub struct PhoneRegisterReq {
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    #[validate(length(min = 5, max = 20, message = "用户名必须在5到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 6, message = "密码至少需要6个字符"))]
    pub password: String,
    #[validate(range(min = 1, max = 3, message = "性别;1:男,2:女,3:保密"))]
    pub gender: i8,
    #[validate(range(min = 18, max = 100, message = "年龄必须在18到100岁之间"))]
    pub age: Option<i32>,
    pub birthday: Option<String>,
    pub avatar: Option<String>,
    pub captcha_id: String,
    pub captcha: String,
}

// 自定义电话号码验证函数
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let phone_regex =
        Regex::new(r"^(13[0-9]|14[01456879]|15[0-35-9]|16[2567]|17[0-8]|18[0-9]|19[0-35-9])\d{8}$")
            .map_err(|_err| ValidationError::new("invalid phone"))?;
    if !phone_regex.is_match(phone) {
        return Err(ValidationError::new("invalid phone"));
    }
    Ok(())
}

/// 注册邮件用户
#[derive(Serialize, Deserialize, Validate)]
pub struct EmailRegisterReq {
    // TODO 需要支持更多邮箱
    #[validate(
        email,
        contains(pattern = "mail", message = "Email must be valid email address")
    )]
    pub email: String,
    #[validate(length(min = 5, max = 20, message = "用户名必须在5到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 6, message = "密码至少需要6个字符"))]
    pub password: String,
    #[validate(range(min = 1, max = 3, message = "性别;1:男,2:女,3:保密"))]
    pub gender: i8,
    #[validate(range(min = 18, max = 100, message = "年龄必须在18到100岁之间"))]
    pub age: Option<i32>,
    pub birthday: Option<String>,
    pub avatar: Option<String>,
    pub captcha_id: String,
    pub captcha: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use utils::json::struct_to_struct;

    #[test]
    fn test_register_type() {
        let expected = RegisterReq {
            phone: Some("phone".to_owned()),
            email: Some("email".to_owned()),
            register_type: RegisterType::Phone,
            username: "username".to_owned(),
            gender: 11,
            age: 12,
            birthday: Some("birthday".to_owned()),
            password: "password".to_owned(),
            avatar: Some("avatar".to_owned()),
            captcha_id: "captcha_id".to_owned(),
            captcha: "captcha".to_owned(),
        };
        let body = json! ({
            "phone": "phone",
            "email": "email",
            "register_type": "phone",
            "username": "username",
            "gender": 11,
            "age": 12,
            "birthday": "birthday",
            "password": "password",
            "avatar": "avatar",
            "captcha_id":"captcha_id",
            "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();
        println!("result: {:#?}", result);
        assert!(expected == result)
    }

    #[test]
    fn test_register_type_by_none() {
        let expected = RegisterReq {
            phone: Some("phone".to_owned()),
            email: None,
            register_type: RegisterType::Phone,
            username: "username".to_owned(),
            gender: 11,
            age: 12,
            birthday: None,
            password: "password".to_owned(),
            avatar: None,
            captcha_id: "captcha_id".to_owned(),
            captcha: "captcha".to_owned(),
        };
        let body = json! ({
            "phone": "phone",
            "email": null,
            "register_type": "phone",
            "username": "username",
            "gender": 11,
            "age": 12,
            "birthday": null,
            "password": "password",
            "avatar": "avatar",
            "captcha_id":"captcha_id",
            "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();
        println!("result: {:#?}", result);
        assert!(expected == result)
    }

    #[test]
    fn test_register_type_by_missing_field() {
        let expected = RegisterReq {
            phone: Some("phone".to_owned()),
            email: None,
            register_type: RegisterType::Phone,
            username: "username".to_owned(),
            gender: 11,
            age: 12,
            birthday: None,
            password: "password".to_owned(),
            avatar: None,
            captcha_id: "".to_owned(),
            captcha: "".to_owned(),
        };
        let body = json! ({
            "phone": "phone",
            "email": null,
            "register_type": "phone",
            "username": "username",
            "gender": 11,
            "age": 12,
            "birthday": null,
            "password": "password",
            "avatar": null,
            // "captcha_id":"captcha_id",
            // "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();
        println!("result: {:#?}", result);
        assert!(expected == result)
    }
}
