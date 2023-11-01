//! 路由层
use crate::middleware;

pub mod log;
pub mod perm_user;
pub mod resources;
pub mod web_site;
pub mod welcome;

use actix_web::middleware::Logger;
use actix_web::{dev::HttpServiceFactory, web};
use tracing_actix_web::TracingLogger;

/// API 服务
/// 注册路由
pub fn register_api() -> impl HttpServiceFactory {
    web::scope("/api")
        .wrap(Logger::default())
        .wrap(TracingLogger::default())
        .wrap(middleware::cors::wrap_cors())
        .wrap(middleware::auth::SayHi)
        .service(
            web::scope("/v1")
                // 打招呼
                .service(welcome::register())
                // 用户管理
                .service(perm_user::register())
                // 系统日志管理
                .service(log::system::register()),
        )
}
