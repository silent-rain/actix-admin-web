//! 数据库
use std::time::Duration;

use code::Error;

use async_trait::async_trait;
pub use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};

#[async_trait]
pub trait DBRepo {}

/// 数据库连接池
#[derive(Debug, Clone)]
pub struct Pool {
    /// 只读实例
    pub rdb: DatabaseConnection,
    /// 读写实例
    pub wdb: DatabaseConnection,
}

// 初始化数据库
pub async fn init(rdb_url: String, wdb_url: String) -> Result<Pool, Error> {
    let rdb = connect(rdb_url).await?;
    let wdb = connect(wdb_url).await?;
    let pool = Pool { rdb, wdb };
    Ok(pool)
}

/// 连接数据库
pub async fn connect(db_url: String) -> Result<DatabaseConnection, Error> {
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    Database::connect(opt)
        .await
        .map_err(|err| Error::DbConnectionError(err.to_string()))
}

#[allow(unused)]
impl Pool {
    /// 获取只读数据库实例
    pub fn rdb(&self) -> &sea_orm::DatabaseConnection {
        &self.rdb
    }
    /// 获取读写数据库实例
    pub fn wdb(&self) -> &sea_orm::DatabaseConnection {
        &self.wdb
    }
    /// 关闭数据库
    async fn close(&self) -> Result<(), Error> {
        self.rdb
            .clone()
            .close()
            .await
            .map_err(|_e| Error::DbCloseError)?;

        self.rdb
            .clone()
            .close()
            .await
            .map_err(|_e| Error::DbCloseError)
    }
}
