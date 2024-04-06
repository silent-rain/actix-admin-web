//! 日志级别

use serde::{Deserialize, Deserializer, Serializer};
use tracing::Level;

/// 字符串反序列化为 tracing::Level
pub fn str_to_level<'de, D>(deserializer: D) -> Result<tracing::Level, D::Error>
where
    D: Deserializer<'de>,
{
    let level = String::deserialize(deserializer)?;
    let t_level = match level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::WARN,
    };
    Ok(t_level)
}

/// tracing::Level 序列化为字符串
pub fn level_to_str<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 将 tracing::Level 转换为一个字符串切片
    let level_str = level.as_str();
    // 调用 serializer 的 serialize_str 方法
    serializer.serialize_str(level_str)
}

// impl From<Level> for String {
//     fn from(level: Level) -> Self {
//         match level.0 {
//             tracing::Level::TRACE => "trace".into(),
//             tracing::Level::DEBUG => "debug".into(),
//             tracing::Level::INFO => "info".into(),
//             tracing::Level::WARN => "warn".into(),
//             tracing::Level::ERROR => "error".into(),
//         }
//     }
// }
