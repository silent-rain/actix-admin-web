//! Mysql 数据库配置
use database::DbOptions;

use serde::{Deserialize, Serialize};

/// Mysql 数据库配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Mysql {
    /// 数据库自动迁移
    pub migrator: bool,
    /// 数据库参数
    pub options: DbOptions,
    /// 只读数据库账号配置
    pub read: MysqlAuth,
    /// 读写数据库账号配置
    pub write: MysqlAuth,
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
        // 这些参数会导致连接失败: ?charset=utf8mb4&parseTime=false&loc=Asia%2FShanghai
        // loc=Local
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name,
        )
    }
}
