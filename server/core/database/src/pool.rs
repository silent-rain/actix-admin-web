//! 数据库连接池
use std::time::Duration;

use code::Error;

pub use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};

/// 数据库特征
pub trait DbRepo {
    /// 获取只读数据库实例
    fn rdb(&self) -> &DatabaseConnection;
    /// 获取读写数据库实例
    fn wdb(&self) -> &DatabaseConnection;
}

/// 数据库连接池
#[derive(Debug, Clone)]
pub struct Pool {
    /// 只读实例
    pub rdb: DatabaseConnection,
    /// 读写实例
    pub wdb: DatabaseConnection,
}

impl Pool {
    /// 初始化数据库连接池
    pub async fn init(rdb_url: String, wdb_url: String) -> Result<Pool, Error> {
        let rdb = Self::connect(rdb_url).await?;
        let wdb = Self::connect(wdb_url).await?;
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
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);
        let db = Database::connect(opt)
            .await
            .map_err(|err| Error::DbConnectionError(err.to_string()))?;

        // 检查连接是否有效
        db.ping()
            .await
            .map_err(|err| Error::DbConnectionAcquire(err.to_string()))?;
        Ok(db)
    }

    /// 从连接生成连接池
    pub fn form_connect(rdb: DatabaseConnection, wdb: DatabaseConnection) -> Pool {
        Pool { rdb, wdb }
    }

    /// 关闭数据库
    pub async fn close(&self) -> Result<(), Error> {
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

impl DbRepo for Pool {
    /// 获取只读数据库实例
    fn rdb(&self) -> &DatabaseConnection {
        &self.rdb
    }
    /// 获取读写数据库实例
    fn wdb(&self) -> &DatabaseConnection {
        &self.wdb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let db_url = "sqlite://../../data.dat?mode=rwc";
        let db = Pool::connect(db_url.to_owned()).await.unwrap();
        let _ = db.close().await;
    }
}
