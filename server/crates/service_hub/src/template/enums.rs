//! 枚举
use serde::{Deserialize, Deserializer, Serialize};

/// 模板状态
#[derive(Debug, Clone, PartialEq, Serialize)]
#[repr(i8)]
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

impl<'de> Deserialize<'de> for AppTemplateStatus {
    fn deserialize<D>(deserializer: D) -> Result<AppTemplateStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = i8::deserialize(deserializer)?;
        match code {
            0 => Ok(AppTemplateStatus::Disabled),
            1 => Ok(AppTemplateStatus::Enabled),
            _ => Err(serde::de::Error::custom("invalid status")),
        }
    }
}
