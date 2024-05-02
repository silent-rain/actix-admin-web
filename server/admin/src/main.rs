//! 程序入口

use std::sync::Arc;

mod asset;
mod config;
mod router;
mod server;

use crate::asset::{AssetAdminWebDist, AssetConfigFile, AssetDbDataFile};

use app_state::{AppState, AssetState};
use service_hub::{
    inject::InjectProvider,
    schedule::task::{TaskRegister, TaskShutdown},
};

use colored::Colorize;
use dotenv::dotenv;
use tokio::sync::RwLock;
use tracing::{info, warn};

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
    let db = database::Pool::init(
        database_url.clone(),
        database_url,
        conf.mysql.options.clone(),
    )
    .await
    .expect("初始化数据库失败");

    // if conf.mysql.migrator {
    //     // 库表迁移器
    //     if let Err(e) = Migrator::up(db.wdb(), None).await {
    //         error!("表迁移失败. err: {e}");
    //     }
    // }
    TaskRegister::start(db.clone());
    // let current = Handle::current();
    // let t_db = db.clone();
    // current.spawn(async {
    //     let mut task = TaskRegister::new(t_db);
    //     task.init().await.expect("定时任务初始化失败");
    // });

    // 共享状态
    let app_state = AppState {};
    let asset_state = Arc::new(AssetState {
        admin_web_dist: RwLock::new(Box::new(AssetAdminWebDist)),
        config_file: RwLock::new(Box::new(AssetConfigFile)),
        db_data_file: RwLock::new(Box::new(AssetDbDataFile)),
    });

    // Using an Arc to share the provider across multiple threads.
    let provider = InjectProvider::anew(db.clone());

    // 启动服务, 并阻塞
    if let Err(e) = server::start(app_state, asset_state, provider, conf).await {
        panic!("server start faild. err: {e}");
    }
    info!("close service...");

    TaskShutdown::shutdown().await.expect("关闭定时任务失败");

    // 关闭数据库
    let _ = db.close().await;
    info!("close database...");

    warn!("{}", "See you again~".yellow());
    Ok(())
}
