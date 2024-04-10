//! 枚举

/// 用户状态
#[derive(Debug, PartialEq)]
pub enum UserStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 用户登陆状态
#[derive(Debug, PartialEq)]
pub enum UserLoginStatus {
    /// 失败
    Failed = 0,
    /// 成功
    Success = 1,
}
