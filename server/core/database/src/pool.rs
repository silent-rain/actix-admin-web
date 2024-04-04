//! 数据库连接池
use std::time::Duration;

use crate::config::DbOptions;

use code::Error;

pub use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, ConnectionTrait, Database};

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
    pub async fn init(rdb_url: String, wdb_url: String, options: DbOptions) -> Result<Pool, Error> {
        let rdb = Self::connect(rdb_url, options.clone()).await?;
        let wdb = Self::connect(wdb_url, options).await?;
        let pool = Pool { rdb, wdb };
        Ok(pool)
    }

    /// 连接数据库
    pub async fn connect(db_url: String, options: DbOptions) -> Result<DatabaseConnection, Error> {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(options.max_connections)
            .min_connections(options.max_connections)
            .connect_timeout(Duration::from_secs(options.connect_timeout))
            .acquire_timeout(Duration::from_secs(options.acquire_timeout))
            .idle_timeout(Duration::from_secs(options.idle_timeout))
            .max_lifetime(Duration::from_secs(options.max_lifetime))
            .sqlx_logging(options.logging_enable)
            .sqlx_logging_level(options.logging_level.into());
        let db = Database::connect(opt)
            .await
            .map_err(|err| Error::DbConnectionError(err.to_string()))?;

        // 检查连接是否有效
        db.ping()
            .await
            .map_err(|err| Error::DbConnectionAcquire(err.to_string()))?;

        // 设置 Time Zone
        // Self::set_time_zone(&db).await?;

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

    /// 设置 Time Zone
    async fn set_time_zone(db: &DatabaseConnection) -> Result<(), Error> {
        let stmt = sea_orm::Statement::from_string(
            db.get_database_backend(),
            "SET time_zone = '+08:00'".to_owned(),
        );
        db.execute(stmt)
            .await
            .map_err(|err| Error::DbTimeZoneError(err.to_string()))?;
        Ok(())
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
        let options = DbOptions::default();
        let db = Pool::connect(db_url.to_owned(), options).await.unwrap();
        let _ = db.close().await;
    }
}
