//! 注册

use std::str::FromStr;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

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
