//! 枚举
use serde::{Deserialize, Serialize};

/// 角色状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum RoleStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 用户状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum UserStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 性别
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(i8)]
pub enum Gender {
    /// 男
    Male = 0,
    /// 女
    Female = 1,
    /// 保密
    Confidential = 2,
}

/// 部门状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum DeptStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// OpenApi接口状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum OpenApiStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// OpenApi接口类别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum OpenApiCategory {
    /// 目录
    Directory = 0,
    /// 接口
    Interface = 1,
}
