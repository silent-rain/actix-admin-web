//! Mysql 数据库配置
use serde::{Deserialize, Serialize};

/// Mysql 数据库配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Mysql {
    pub options: Options,
    pub read: MysqlAuth,
    pub write: MysqlAuth,
}

/// 参数配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Options {
    pub max_connections: i32, // 最大打开的连接数
    pub max_lifetime: i32,    // 设置最大连接超时(min)
    pub enable_log: bool,     // 是否开启 SQL 日志
}

/// 权限配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MysqlAuth {
    pub key: String,      // db信息唯一标识
    pub host: String,     // IP或域名
    pub port: i32,        // 端口
    pub username: String, // 账号
    pub password: String, // 密码
    pub db_name: String,  // 数据库名称
}

impl MysqlAuth {
    /// 数据库地址
    pub fn dns(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.db_name,
        )
    }
}
