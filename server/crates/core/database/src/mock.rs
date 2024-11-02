//! Mock 模拟测试

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Schema, Statement};
use sea_orm_migration::{MigrationTrait, SchemaManager};

use crate::{config::Level, DbOptions, Pool, PoolTrait};

#[derive(Debug, Default)]
pub struct Mock {}

impl Mock {
    /// 从迁移文件创建表
    pub async fn from_migration(
        migration: &dyn MigrationTrait,
    ) -> Result<Arc<dyn PoolTrait>, DbErr> {
        let pool = Self::connect().await;

        let manager = SchemaManager::new(pool.db());
        migration.up(&manager).await?;

        Ok(pool)
    }

    /// 从多个迁移文件创建表
    pub async fn from_migrations(
        migrations: Vec<&dyn MigrationTrait>,
    ) -> Result<Arc<dyn PoolTrait>, DbErr> {
        let pool = Self::connect().await;

        for migration in migrations {
            let manager = SchemaManager::new(pool.db());
            migration.up(&manager).await?;
        }

        Ok(pool)
    }

    /// 从实体创建表
    pub async fn from_entity<E: EntityTrait>(entity: E) -> Result<Arc<dyn PoolTrait>, DbErr> {
        let pool = Self::connect().await;

        let builder = pool.db().get_database_backend();
        let schema = Schema::new(builder);
        pool.db()
            .execute(builder.build(&schema.create_table_from_entity(entity)))
            .await?;

        Ok(pool)
    }

    /// 从实体创建表
    pub async fn from_str(sql: &str) -> Result<Arc<dyn PoolTrait>, DbErr> {
        let pool = Self::connect().await;

        let stmt = Statement::from_sql_and_values(pool.db().get_database_backend(), sql, []);
        pool.db().execute(stmt).await?;

        Ok(pool)
    }

    /// 连接数据库
    pub async fn connect() -> Arc<dyn PoolTrait> {
        // Connecting SQLite
        let db_url = "sqlite::memory:".to_string();
        let opt = DbOptions {
            logging_enable: true,
            logging_level: Level::Info,
            ..Default::default()
        };
        let db = Pool::connect(db_url, opt).await.expect("db init failed");
        let pool = Pool::form_connect(db);

        Arc::new(pool)
    }
}
