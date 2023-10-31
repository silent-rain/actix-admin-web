//! 路由层
use crate::middleware;

pub mod perm_user;
pub mod resources;
pub mod web_site;
pub mod welcome;

use actix_web::middleware::Logger;
use actix_web::Scope;
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
                .service(welcome_routes())
                // 用户管理
                .service(user_routes()),
        )
}

/// 打招呼
fn welcome_routes() -> Scope {
    web::scope("/greet").route("", web::get().to(welcome::Routes::greet))
}

/// 用户管理
fn user_routes() -> Scope {
    web::scope("/user")
        .route("/list", web::get().to(perm_user::Routes::list))
        .route("/info", web::get().to(perm_user::Routes::info))
        .route("/add", web::post().to(perm_user::Routes::add))
}
