//! 服务

use crate::{
    app::public::WebSiteRouter, config::AppConfig, inject::AProvider, router, state::AppState,
};

use actix_web::{http::KeepAlive, web, App, HttpServer};
use listenfd::ListenFd;
use tracing::{error, warn};

/// 启动服务
pub async fn start(
    app_state: AppState,
    config: AppConfig,
    provider: AProvider,
    server_url: &str,
) -> std::io::Result<()> {
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(provider.clone()))
            .app_data(web::Data::new(config.clone()))
            // API 服务
            .service(router::register())
            // 静态资源
            .service(WebSiteRouter::register())
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
