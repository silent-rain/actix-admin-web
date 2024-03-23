//! 程序入口
use std::sync::Arc;

mod asset;
mod config;
pub mod inject;
mod middleware;
mod server;
mod state;

// pub mod controller;
// pub mod dao;
// pub mod dto;
// pub mod routes;
// pub mod service;

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
    if let Err(e) = config::init("config.toml") {
        log::error!("配置文件加载失败, err: {e}");
        return Ok(());
    }
    let cfg = config::instance();

    // 初始化日志
    let _guards = logger::Logger::build(&cfg.logger).expect("初始化日志失败");

    // mysql dns
    let database_url = cfg.mysql.write.dns();
    // sqlite dns
    // let database_url = cfg.sqlite.dns();

    // 初始化数据库
    let db = database::Pool::init(database_url.clone(), database_url)
        .await
        .expect("初始化数据库失败");

    if cfg.mysql.migrator {
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
    let address = cfg.server.base.address();
    if let Err(e) = server::start(app_state.clone(), provider, &address).await {
        error!("server start faild. err: {e}");
    }

    // 关闭数据库
    let _ = db.close().await;

    warn!("See you again~");
    Ok(())
}
