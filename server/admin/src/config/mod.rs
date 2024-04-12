//! 配置文件
#![allow(unused)]

use std::fs::read_to_string;
use std::sync::OnceLock;

pub mod environment;
pub mod mysql;
pub mod server;
pub mod sqlite;

use code::Error;
pub use logger::config::Logger;

use serde::{Deserialize, Serialize};
use tracing::error;

/// 全局配置对象
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = init("./config.yaml");
/// assert!(config.is_ok());
/// ```
pub fn init(path: &str) -> Result<AppConfig, Error> {
    let content = read_to_string(path)?;
    let config: AppConfig = serde_yaml::from_str(&content).map_err(|e| {
        error!("{}, err: {e}", Error::ConfigParseError);
        eprintln!("{:#?}", e);
        Error::ConfigParseError
    })?;
    GLOBAL_CONFIG.get_or_init(|| config.clone());
    Ok(config)
}

/// 获取全局配置
/// # Examples
/// ```
/// config = instance()
/// assert!(config.is_ok());
/// ```
pub fn instance() -> &'static AppConfig {
    let config = GLOBAL_CONFIG.get();
    match config {
        Some(config) => config,
        None => {
            panic!("configuration not initialized!")
        }
    }
}

/// 全局配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// 环境配置
    #[serde(default)]
    pub environment: environment::Environment,
    /// 服务配置
    #[serde(default)]
    pub server: server::Server,
    /// Sqlite3 数据库配置
    #[serde(default)]
    pub sqlite: sqlite::Sqlite,
    /// Mysql 数据库配置
    #[serde(default)]
    pub mysql: mysql::Mysql,
    /// 日志配置
    #[serde(default)]
    pub logger: Logger,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let path = "config.yaml";
        let config = init(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../../config.yaml");
        assert_ne!(yaml_str, "");
    }
}
