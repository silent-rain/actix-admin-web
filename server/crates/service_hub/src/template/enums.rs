//! 枚举
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 模板状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum AppTemplateStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}
