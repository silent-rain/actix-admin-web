//! 程序入口
mod asset;
mod context;
mod middleware;
mod state;

mod controller;
mod routes;

use actix_web::{http::KeepAlive, web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use tracing::{error, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 读取配置环境变量
    dotenv().ok();

    // 加载配置文件
    if let Err(e) = config::init("config.toml") {
        log::error!("配置文件加载失败. err: {e}");
        return Ok(());
    }

    let cfg = config::instance();
    warn!("print config:\n{:#?}", cfg);

    // 初始化日志
    let _guards = logger::init(cfg.logger.clone()).expect("初始化日志失败");

    // mysql dns
    // let database_url = cfg.mysql.write.dns();
    // sqlite dns
    let database_url = cfg.sqlite.dns();

    // 初始化数据库
    let db = database::init(database_url.clone(), database_url)
        .await
        .expect("初始化数据库失败");

    // 应用中运行迁移器
    // if let Err(e) = Migrator::up(db.wdb(), None).await {
    //     log::error!("表迁移失败. err: {e}");
    // }

    // 共享状态
    let app_state = state::AppState { db: db.clone() };

    // 启动服务, 并阻塞
    let address = cfg.server.base.address();
    if let Err(e) = server(app_state.clone(), &address).await {
        error!("server start faild. err: {e}");
    }

    warn!("See you again~");
    Ok(())
}

// 启动服务
async fn server(app_state: state::AppState, server_url: &str) -> std::io::Result<()> {
    let mut server = HttpServer::new(move || {
        let context = context::Context {
            ..Default::default()
        };

        App::new()
            .app_data(web::Data::new(context))
            .app_data(web::Data::new(app_state.clone()))
            // API 服务
            .service(routes::register_api_routes())
            // 内嵌 静态资源
            .service(routes::register_web_routes())
    })
    // 保持连接打开状态以等待后续请求, 使用操作系统保持活动状态
    .keep_alive(KeepAlive::Os)
    // 自动启动多个 HTTP 工作线程，默认情况下，此数字等于系统中物理 CPU 的数量。
    .workers(num_cpus::get());

    // 是否存在套接字
    let mut listenfd = ListenFd::from_env();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    // 打印服务地址
    warn!("Starting server at http://{server_url}");

    // 启动服务
    if let Err(e) = server.run().await {
        error!("server colse faild. err: {e}");
    }
    Ok(())
}
