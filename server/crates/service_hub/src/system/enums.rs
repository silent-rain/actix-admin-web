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
