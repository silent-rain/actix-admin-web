//!Sqlite3 数据库配置
use serde::{Deserialize, Serialize};

/// Sqlite3 数据库配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sqlite {
    pub filepath: String,     // 数据库路径
    pub pool_min_idle: u32,   // 最小连接数
    pub pool_max_open: u32,   // 最大连接数
    pub timeout_seconds: u64, // 连接超时时间单位秒
}

impl Default for Sqlite {
    fn default() -> Sqlite {
        Sqlite {
            filepath: "data.dat".to_string(),
            pool_min_idle: 8,
            pool_max_open: 32,
            timeout_seconds: 15,
        }
    }
}

impl Sqlite {
    /// 数据库地址
    pub fn dns(&self) -> String {
        format!("sqlite://{}", self.filepath)
    }
}
