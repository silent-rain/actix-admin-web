//! Web 静态资源服务
use crate::controller::resources;
use crate::utils::open_api_doc::ApiDoc;

use actix_web::{dev::HttpServiceFactory, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// WEB 服务
pub fn register() -> impl HttpServiceFactory {
    web::scope("")
        // .wrap(actix_web::middleware::NormalizePath::default())
        // 用于压缩响应有效负载的中间件。
        .wrap(actix_web::middleware::Compress::default())
        // 文件服务
        // .service(Files::new("/", "../web/dist").show_files_listing())
        // .service(
        //     Files::new("/", "../web/dist")
        //         .index_file("index.html")
        //         .prefer_utf8(true),
        // )
        // swagger-ui
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        // WEB 静态资源服务
        .service(web::scope("").route(
            "/{filename:.*}",
            web::get().to(resources::Controller::index),
        ))
}
