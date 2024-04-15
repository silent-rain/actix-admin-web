//! 数据库库表迁移
use std::env;

use colored::Colorize;
use database::{DbOptions, DbRepo};
use logger::config::{ConsoleConfig, Level, Logger};
use migration::{Migrator, MigratorTrait};

use dotenv::dotenv;
use tracing::{error, info};

// const DATABASE_URL: &str = "sqlite://data.dat";
// const DATABASE_URL: &str = "mysql://user:pass@127.0.0.1:3306/db_name";

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
    let options = DbOptions::default();
    let db = database::Pool::init(database_url.clone(), database_url, options)
        .await
        .expect("初始化数据库失败");

    // 库表迁移器
    if let Err(e) = Migrator::up(db.wdb(), None).await {
        error!("表迁移失败. err: {e}");
    }

    info!("{}", "库表迁移完毕...".green());
    Ok(())
}
