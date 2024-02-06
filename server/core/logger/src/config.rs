//! 配置

use serde::{Deserialize, Serialize};

/// 日志配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Logger {
    /// 终端配置
    #[serde(default)]
    pub console: ConsoleOptions,
    pub console_bunyan: ConsoleBunyanOptions,
    /// 文件配置
    #[serde(default)]
    pub file: FileOptions,
    /// 数据库配置
    #[serde(default)]
    pub db: DbOptions,
}

/// 终端配置参数
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConsoleOptions {
    /// 日志级别, trace/debug/info/warn/error
    pub level: Level,
    /// 是否启用，默认不启用
    pub enable: bool,
}

impl Default for ConsoleOptions {
    fn default() -> Self {
        Self {
            level: Level::Warn,
            enable: false,
        }
    }
}

/// Bunyan 终端配置参数
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConsoleBunyanOptions {
    /// 日志级别, trace/debug/info/warn/error
    pub level: Level,
    /// 是否启用，默认不启用
    pub enable: bool,
}

impl Default for ConsoleBunyanOptions {
    fn default() -> Self {
        Self {
            level: Level::Warn,
            enable: false,
        }
    }
}

/// 文件配置参数
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileOptions {
    /// 文件路径
    #[serde(default)]
    pub filepath: String,
    /// 文件名称
    pub filename: String,
    /// 日志级别, trace/debug/info/warn/error
    pub level: Level,
    /// 是否启用，默认不启用
    pub enable: bool,
}

impl Default for FileOptions {
    fn default() -> Self {
        Self {
            filepath: "logs".to_owned(),
            filename: "app.log".to_owned(),
            level: Level::Warn,
            enable: false,
        }
    }
}

/// 数据库配置参数
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DbOptions {
    /// 数据库地址
    pub address: String,
    /// 日志记录器名称
    pub log_name: String,
    /// 日志级别, trace/debug/info/warn/error
    pub level: Level,
    /// 是否启用，默认不启用
    pub enable: bool,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            address: "".to_owned(),
            log_name: "db_layer".to_owned(),
            level: Level::Warn,
            enable: false,
        }
    }
}

/// 日志级别
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Level {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

impl Default for Level {
    fn default() -> Self {
        Self::Warn
    }
}

// Level 别转换为 tracing::Level
impl From<Level> for tracing::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => tracing::Level::TRACE,
            Level::Debug => tracing::Level::DEBUG,
            Level::Info => tracing::Level::INFO,
            Level::Warn => tracing::Level::WARN,
            Level::Error => tracing::Level::ERROR,
        }
    }
}

/// 字符串转为 Level
impl From<String> for Level {
    fn from(level: String) -> Self {
        match level.as_str() {
            "trace" => Level::Trace,
            "debug" => Level::Debug,
            "info" => Level::Info,
            "warn" => Level::Warn,
            "error" => Level::Error,
            _ => Level::Warn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_file_options() {
        let text = r#"
        {
            "filepath": "logs",
            "enable": true
        }
        "#;
        let options = serde_json::from_str::<FileOptions>(text).unwrap();
        println!("{:#?}", options);
        let ac = FileOptions {
            filepath: "logs".to_owned(),
            level: Level::Warn,
            enable: true,
            ..Default::default()
        };
        assert!(options == ac)
    }
}
