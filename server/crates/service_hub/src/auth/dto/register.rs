//! 注册

use crate::auth::enums::RegisterType;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 注册用户
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegisterReq {
    /// 手机号码
    // #[validate(custom(function = "validate_phone"))]
    pub phone: Option<String>,
    /// 邮箱
    // #[validate(email)]
    pub email: Option<String>,
    /// 注册用户类型
    pub register_type: RegisterType,
    /// 用户名称
    #[validate(length(min = 5, max = 20, message = "用户名必须在5到20个字符之间"))]
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别, 0:男,1:女,2:保密
    #[validate(range(min = 0, max = 3, message = "性别, 0:男,1:女,2:保密"))]
    pub gender: i8,
    /// 年龄
    #[validate(range(min = 18, max = 100, message = "年龄必须在18到100岁之间"))]
    pub age: Option<i32>,
    /// 出生日期
    pub birthday: Option<String>,
    /// 密码
    #[validate(length(min = 6, message = "密码至少需要6个字符"))]
    pub password: String,
    /// 头像URL
    pub avatar: Option<String>,
    /// 验证码ID
    #[serde(default)]
    pub captcha_id: String,
    /// 验证码
    #[serde(default)]
    pub captcha: String,
}

#[cfg(test)]
mod tests {
    use crate::auth::enums::RegisterType;

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
            real_name: Some("real_name".to_owned()),
            gender: 11,
            age: Some(12),
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
            "real_name": "real_name",
            "gender": 11,
            "age": 12,
            "birthday": "birthday",
            "password": "password",
            "avatar": "avatar",
            "captcha_id":"captcha_id",
            "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();
        println!("expected: {:#?}", expected);
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
            real_name: Some("real_name".to_owned()),
            gender: 1,
            age: Some(12),
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
            "real_name": "real_name",
            "gender": 1,
            "age": 12,
            "birthday": null,
            "password": "password",
            "avatar": null,
            "captcha_id":"captcha_id",
            "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();

        assert!(expected == result)
    }

    #[test]
    fn test_register_type_by_missing_field() {
        let expected = RegisterReq {
            phone: Some("phone".to_owned()),
            email: None,
            register_type: RegisterType::Phone,
            username: "username".to_owned(),
            real_name: Some("real_name".to_owned()),
            gender: 11,
            age: Some(12),
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
            "real_name": "real_name",
            "gender": 11,
            "age": 12,
            "birthday": null,
            "password": "password",
            "avatar": null,
            // "captcha_id":"captcha_id",
            // "captcha": "captcha",
        });
        let result: RegisterReq = struct_to_struct(&body).unwrap();
        println!("expected: {:#?}", expected);
        println!("result: {:#?}", result);
        assert!(expected == result)
    }
}
