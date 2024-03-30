//! 程序入口
use std::sync::Arc;

mod asset;
mod config;
pub mod inject;
mod middleware;
mod server;
mod state;

pub mod app;
pub mod router;

use database::DbRepo;
use migration::{Migrator, MigratorTrait};

use dotenv::dotenv;
use tracing::{error, warn};

use inject::{AProvider, Provider};

/// 程序入口
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 读取配置环境变量
    dotenv().ok();

    // 加载配置文件
    let conf = match config::init("config.yaml") {
        Ok(v) => v,
        Err(err) => {
            panic!("配置文件加载失败, err: {err}")
        }
    };

    // 初始化日志
    let _guards = logger::Logger::build(&conf.logger).expect("初始化日志失败");

    // mysql dns
    let database_url = conf.mysql.write.dns();
    // sqlite dns
    // let database_url = conf.sqlite.dns();

    // 初始化数据库
    let db = database::Pool::init(database_url.clone(), database_url)
        .await
        .expect("初始化数据库失败");

    if conf.mysql.migrator {
        // 库表迁移器
        if let Err(e) = Migrator::up(db.wdb(), None).await {
            error!("表迁移失败. err: {e}");
        }
    }

    // 共享状态
    let app_state = state::AppState { db: db.clone() };

    // Using an Arc to share the provider across multiple threads.
    let provider: AProvider = Arc::new(Provider::new(db.clone()));

    // 启动服务, 并阻塞
    let address = conf.server.base.address();
    if let Err(e) = server::start(app_state.clone(), conf, provider, &address).await {
        panic!("server start faild. err: {e}");
    }

    // 关闭数据库
    let _ = db.close().await;

    warn!("See you again~");
    Ok(())
}
