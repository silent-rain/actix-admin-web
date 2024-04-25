//! 枚举

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// 注册用户类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegisterType {
    /// 手机号码
    #[serde(rename = "phone")]
    Phone,
    /// 邮箱
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

impl From<RegisterType> for String {
    fn from(value: RegisterType) -> Self {
        match value {
            RegisterType::Phone => "phone".to_owned(),
            RegisterType::Email => "email".to_owned(),
        }
    }
}
