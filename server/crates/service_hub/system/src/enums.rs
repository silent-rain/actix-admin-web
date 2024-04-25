//! 枚举
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 配置状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ConfigStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 字典维度状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum DictDimStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 字典数据状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum DictDataStatus {
    /// 停用
    Disabled = 0,
    /// 正常
    Enabled = 1,
}

/// 验证码状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum CaptchaStatus {
    /// 无效验证码
    Invalid = 0,
    /// 有效验证码
    Valid = 1,
}

/// ICON图片扩展类型,svg,png
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum ImageType {
    /// 无效验证码
    #[serde(rename = "svg")]
    Svg,
    /// 有效验证码
    #[serde(rename = "png")]
    Png,
}

impl From<ImageType> for String {
    fn from(value: ImageType) -> Self {
        match value {
            ImageType::Svg => "svg".to_owned(),
            ImageType::Png => "png".to_owned(),
        }
    }
}
