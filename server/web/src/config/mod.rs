//! 配置文件
#![allow(unused)]

use std::fs::read_to_string;
use std::sync::Arc;

pub mod environment;
pub mod mysql;
pub mod server;
pub mod sqlite;

use code::Error;
pub use logger::config::Logger;

use log::error;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

/// 全局配置对象
static GLOBAL_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = init("./config.toml");
/// assert!(config.is_ok());
/// ```
pub fn init(path: &str) -> Result<(), Error> {
    let content = read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content).map_err(|e| {
        error!("{}, err: {e}", Error::ConfigParseError);
        eprintln!("{:#?}", e);
        Error::ConfigParseError
    })?;
    GLOBAL_CONFIG.get_or_init(|| Arc::new(config));
    Ok(())
}

/// 获取全局配置
/// # Examples
/// ```
/// config = instance()
/// assert!(config.is_ok());
/// ```
pub fn instance() -> Arc<AppConfig> {
    let config = GLOBAL_CONFIG.get();
    match config {
        Some(config) => Arc::clone(config),
        None => {
            log::error!("configuration not initialized!");
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
        let path = "../../config.toml";
        let config = init(path);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../../config.toml");
        assert_ne!(yaml_str, "");
    }
}
