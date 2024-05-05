//! 枚举

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户登陆状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum UserLoginStatus {
    /// 失败
    Failed = 0,
    /// 成功
    Success = 1,
}

/// 用户登陆禁用状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum UserLoginDisabledStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// Api 操作日志类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpType {
    /// 请求
    #[serde(rename = "REQ")]
    Req,
    /// 响应
    #[serde(rename = "RSP")]
    Rsp,
}

impl From<HttpType> for String {
    fn from(value: HttpType) -> Self {
        match value {
            HttpType::Req => "REQ".to_owned(),
            HttpType::Rsp => "RSP".to_owned(),
        }
    }
}