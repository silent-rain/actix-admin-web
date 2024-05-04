//! 数据库库表迁移
use std::env;

use database::DbOptions;
use logger::config::{ConsoleConfig, Level, Logger};
use migration::Migrator;

use colored::Colorize;
use dotenv::dotenv;
use sea_orm_migration::MigratorTrait;
use sqlx::{mysql::MySqlPoolOptions, Executor};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取配置环境变量
    dotenv().ok();

    let conf = Logger {
        color_eyre: false,
        console: ConsoleConfig {
            level: Level::Info,
            enable: true,
        },
        console_bunyan: Default::default(),
        file: Default::default(),
        db: Default::default(),
    };
    // 初始化日志
    let _guards = logger::Logger::build(&conf).expect("初始化日志失败");

    // 初始化数据库
    let database_url = env::var("DATABASE_URL").expect("read DATABASE_URL failed");

    // 数据库不存在则创建数据库
    create_db(database_url.clone()).await?;

    // 迁移表
    migrator_up(database_url).await?;

    info!("{}", "库表迁移完毕...".green());
    Ok(())
}

/// 迁移表
async fn migrator_up(database_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let options = DbOptions::default();
    let db = database::Pool::connect(database_url, options)
        .await
        .expect("初始化数据库失败");

    // 库表迁移器
    if let Err(e) = Migrator::up(&db, None).await {
        panic!("表迁移失败. err: {e}");
    }

    Ok(())
}

/// 数据库不存在则创建数据库
async fn create_db(database_url: String) -> Result<(), Box<dyn std::error::Error>> {
    // Sqlite3 无需创建数据库
    if database_url.starts_with("sqlite") {
        return Ok(());
    }

    // 数据库服务器连接信息
    let slash_pos = database_url.rfind('/').unwrap();
    let database_server = &database_url[0..slash_pos];
    let database_name = &database_url[slash_pos + 1..];

    // 创建数据库连接
    let connection = MySqlPoolOptions::new().connect(database_server).await?;

    // 检查数据库是否存在
    let sql = format!(
        "SELECT 1 FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = '{}'",
        database_name
    );
    let row: (Option<i32>,) = match sqlx::query_as(&sql).fetch_one(&connection).await {
        Ok(v) => v,
        Err(err) => match err {
            sqlx::Error::RowNotFound => (None,),
            _ => panic!("查询数据库异常, err:{:#?}", err),
        },
    };

    // 如果数据库存在，则退出
    if row.0.is_some() {
        return Ok(());
    }

    connection
    .execute(sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS `{}` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci", database_name)))
    .await?;

    Ok(())
}
