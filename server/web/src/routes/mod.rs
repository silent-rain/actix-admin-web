//! 路由层
use crate::middleware;

mod log;
pub mod perm;
pub mod web_site;
pub mod welcome;

use log::system;
use perm::perm_role;
use perm::perm_user;

use actix_web::middleware::Logger;
use actix_web::{dev::HttpServiceFactory, web};
use tracing_actix_web::TracingLogger;

/// API 服务
/// 注册路由
pub fn register_api() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        // 中间件
        .wrap(Logger::default())
        .wrap(TracingLogger::default())
        .wrap(middleware::cors::wrap_cors())
        // .wrap(middleware::auth::SayHi)
        // 打招呼
        .service(welcome::register())
        // 用户管理
        .service(perm_user::register())
        // 角色管理
        .service(perm_role::register())
        // 系统日志管理
        .service(system::register())
}
