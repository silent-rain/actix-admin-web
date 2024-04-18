//! 数据库库表迁移
use std::env;

use database::DbOptions;
use logger::config::{ConsoleConfig, Level, Logger};

use colored::Colorize;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use tracing::{error, info};

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
    let db = database::Pool::connect(database_url, options)
        .await
        .expect("初始化数据库失败");

    // 库表迁移器
    if let Err(e) = Migrator::up(&db, None).await {
        error!("表迁移失败. err: {e}");
    }

    info!("{}", "库表迁移完毕...".green());
    Ok(())
}
