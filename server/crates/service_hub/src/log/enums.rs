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

impl Default for UserLoginStatus {
    fn default() -> Self {
        Self::Success
    }
}

impl From<UserLoginStatus> for i8 {
    fn from(value: UserLoginStatus) -> Self {
        value as i8
    }
}
