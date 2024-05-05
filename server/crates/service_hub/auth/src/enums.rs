//! 枚举

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// 注册用户类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRegisterType {
    /// 手机号码
    #[serde(rename = "phone")]
    Phone,
    /// 邮箱
    #[serde(rename = "email")]
    Email,
}

impl Default for UserRegisterType {
    fn default() -> Self {
        Self::Phone
    }
}

/// 实现FromStr trait来定义如何从字符串解析为RegisterType
impl FromStr for UserRegisterType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "phone" => Ok(UserRegisterType::Phone),
            "email" => Ok(UserRegisterType::Email),
            _ => Err(()),
        }
    }
}

impl From<UserRegisterType> for String {
    fn from(value: UserRegisterType) -> Self {
        match value {
            UserRegisterType::Phone => "phone".to_owned(),
            UserRegisterType::Email => "email".to_owned(),
        }
    }
}
