//! 数据库配置
use serde::{Deserialize, Serialize};

/// 参数配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbOptions {
    /// Set the maximum number of connections of the pool
    pub max_connections: u32,
    /// Set the minimum number of connections of the pool
    pub min_connections: u32,
    /// Set the timeout duration when acquiring a connection
    pub connect_timeout: u64,
    /// Set the maximum amount of time to spend waiting for acquiring a connection
    pub acquire_timeout: u64,
    /// Set the idle duration before closing a connection
    pub idle_timeout: u64,
    /// Set the maximum lifetime of individual connections
    pub max_lifetime: u64,
    /// Enable SQLx statement logging (default true)
    pub logging_enable: bool,
    /// Set SQLx statement logging level (default INFO). (ignored if sqlx_logging is false)
    pub logging_level: Level,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            max_connections: 20,
            min_connections: 10,
            connect_timeout: 60,
            acquire_timeout: 60,
            idle_timeout: 60,
            max_lifetime: 60,
            logging_enable: true,
            logging_level: Level::Info,
        }
    }
}

/// 日志级别
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Level {
    #[serde(rename = "off")]
    Off,
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

// String 别转换为 log::LevelFilter
impl From<Level> for log::LevelFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Off => log::LevelFilter::Off,
            Level::Trace => log::LevelFilter::Trace,
            Level::Debug => log::LevelFilter::Debug,
            Level::Info => log::LevelFilter::Info,
            Level::Warn => log::LevelFilter::Warn,
            Level::Error => log::LevelFilter::Error,
        }
    }
}
