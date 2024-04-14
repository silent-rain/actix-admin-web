//! 枚举

use serde::{Deserialize, Serialize};

/// 用户登陆状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserLoginStatus {
    /// 失败
    Failed = 0,
    /// 成功
    Success = 1,
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
