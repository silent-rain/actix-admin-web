//! 数据库连接池
use std::{sync::Arc, time::Duration};

use crate::config::DbOptions;

use code::Error;

pub use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend};

pub type ArcDbRepo = Arc<dyn DbRepo + 'static>;

/// 数据库特征
pub trait DbRepo: Send + Sync {
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
    /// 数据库连接参数:
    /// 2. min_connections: 设置连接池的最小连接数。这是连接池会保持打开的最小数据库连接数量，即使这些连接当前没有被使用。
    /// 3. connect_timeout: 设置连接数据库时的超时时间（以秒为单位）。如果在这段时间内无法建立连接，操作将被取消并返回错误。
    /// 4. acquire_timeout: 设置从连接池获取连接时的超时时间（以秒为单位）。如果在这段时间内无法获取连接，操作将被取消并返回错误。
    /// 5. idle_timeout: 设置连接在被回max_connections: 设置连接池的最大连接数。这是连接池可以打开的最大数据库连接数量。当达到这个数量时，新的连接收之前可以保持空闲状态的最长时间（以秒为单位）。如果连接在这段时间内没有被使用，它将被关闭并从连接池中移除。
    /// 6. max_lifetime: 设置连接的最大生命周期（以秒为单位）。即使连接仍在使用中，超过这个时间后，它也会被关闭并从连接池中移除。
    /// 7. sqlx_logging: 启用或禁用 SQLx 日志记录。SQLx 是 SeaORM 底层使用的数据库驱动，这个选项控制是否记录 SQLx 的日志信息。
    /// 8. sqlx_logging_level: 设置 SQLx 日志记录的级别。这个选项决定了记录哪些级别的日志信息，例如错误、警告、信息或调试信息。
    ///
    /// 查看最大连接数：
    /// ```sql
    /// SHOW VARIABLES LIKE 'max_connections';
    /// ```
    pub async fn connect(db_url: String, options: DbOptions) -> Result<DatabaseConnection, Error> {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(options.max_connections)
            .min_connections(options.min_connections)
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
        Self::set_time_zone(&db).await?;

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
    /// 不支持SQLite3
    #[allow(unused)]
    async fn set_time_zone(db: &DatabaseConnection) -> Result<(), Error> {
        if db.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }
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
        let db_url = "sqlite://./data.dat?mode=rwc";
        let options = DbOptions::default();
        let db = Pool::connect(db_url.to_owned(), options).await.unwrap();
        let _ = db.close().await;
    }
}
