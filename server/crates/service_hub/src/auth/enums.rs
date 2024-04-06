//! 枚举

/// 用户状态
#[derive(Debug, PartialEq)]
pub enum UserStatus {
    /// 启用
    Enabled = 1,
    /// 禁用
    Disabled = 2,
}
