//! 枚举
use serde::{Deserialize, Serialize};

/// 配置状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum ConfigStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 字典维度状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum DictDimStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 字典数据状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum DictDataStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 验证码状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum CaptchaStatus {
    /// 无效验证码
    Invalid = 0,
    /// 有效验证码
    Valid = 1,
}
