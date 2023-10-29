//!路由层
use crate::controller::resources;
use crate::middleware;
mod user;
use crate::controller::welcome;

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
                .service(user::register_routes()) // 角色管理
                // .service(
                //     web::scope("/role")
                //         .route("all", web::to(user::all))
                //         .route("list", web::to(user::list)),
                // ),
                // 打招呼
                .service(web::scope("").route("greet", web::get().to(welcome::Controller::greet))),
        )
}

/// WEB 服务
/// 注册 WEB 静态资源路由
pub fn register_web_routes() -> impl HttpServiceFactory {
    web::scope("")
        // .wrap(actix_web::middleware::NormalizePath::default())
        .wrap(actix_web::middleware::Compress::default())
        // 静态资源
        // 显示文件列表
        // .service(Files::new("/", "../web/dist").show_files_listing())
        // .service(
        //     Files::new("/", "../web/dist")
        //         .index_file("index.html")
        //         .prefer_utf8(true),
        // )
        .service(web::scope("").route(
            "/{filename:.*}",
            web::get().to(resources::Controller::index),
        ))
}
