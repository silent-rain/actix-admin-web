//! 枚举
use serde::{Deserialize, Serialize};

/// 模板状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppTemplateStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

impl Default for AppTemplateStatus {
    fn default() -> Self {
        Self::Enabled
    }
}

impl From<AppTemplateStatus> for i8 {
    fn from(value: AppTemplateStatus) -> Self {
        value as i8
    }
}
