//! 枚举
use serde::{Deserialize, Serialize};

/// 角色状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoleStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

impl Default for RoleStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl From<RoleStatus> for i8 {
    fn from(value: RoleStatus) -> Self {
        value as i8
    }
}

/// 用户状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl From<UserStatus> for i8 {
    fn from(value: UserStatus) -> Self {
        value as i8
    }
}

/// 性别
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Gender {
    /// 男
    Male = 0,
    /// 女
    Female = 1,
    /// 保密
    Confidential = 2,
}

impl Default for Gender {
    fn default() -> Self {
        Self::Male
    }
}

impl From<Gender> for i8 {
    fn from(value: Gender) -> Self {
        value as i8
    }
}

impl From<i8> for Gender {
    fn from(value: i8) -> Gender {
        match value {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Confidential,
            _ => Gender::Male,
        }
    }
}

/// 部门状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeptStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

impl Default for DeptStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl From<DeptStatus> for i8 {
    fn from(value: DeptStatus) -> Self {
        value as i8
    }
}
