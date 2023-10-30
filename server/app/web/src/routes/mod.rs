//! 路由层
use crate::middleware;

pub mod resources;
pub mod perm_user;
pub mod web_site;
pub mod welcome;

use actix_web::middleware::Logger;
use actix_web::{dev::HttpServiceFactory, web};
use tracing_actix_web::TracingLogger;

/// API 服务
/// 注册路由
pub fn register_api_routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .wrap(Logger::default())
        .wrap(TracingLogger::default())
        .wrap(middleware::cors::wrap_cors())
        .wrap(middleware::auth::SayHi)
        .service(
            web::scope("/v1")
                // 用户管理
                .service(perm_user::list)
                .service(perm_user::info)
                .service(perm_user::add)
                // 角色管理
                // .service(
                //     web::scope("/role")
                //         .route("all", web::to(user::all))
                //         .route("list", web::to(user::list)),
                // ),
                // 打招呼
                .service(welcome::greet),
        )
}
