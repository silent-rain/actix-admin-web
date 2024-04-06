//! 服务

use crate::{
    config::AppConfig,
    router::{self, admin_web_site::AdminWebSiteRouter},
    state::AppState,
};

use service_hub::{
    inject::AProvider,
    // public::WebSiteRouter
};

use actix_web::{http::KeepAlive, web, App, HttpServer};
use listenfd::ListenFd;
use tracing::{error, warn};

/// 启动服务
pub async fn start(
    app_state: AppState,
    provider: AProvider,
    config: AppConfig,
) -> std::io::Result<()> {
    let config_s = config.clone();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(provider.clone()))
            .app_data(web::Data::new(config_s.clone()))
            // API 服务
            .service(router::register())
            // 后台管理 WEB 服务
            .service(AdminWebSiteRouter::register())
    })
    // 保持连接打开状态以等待后续请求, 使用操作系统保持活动状态
    .keep_alive(KeepAlive::Os)
    // 自动启动多个 HTTP 工作线程，默认情况下，此数字等于系统中物理 CPU 的数量。
    .workers(num_cpus::get());

    // 服务监听地址
    let server_url = config.server.base.address();

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
