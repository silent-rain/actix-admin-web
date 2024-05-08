//! 后台管理 WEB 服务

use crate::controller::admin_web_site::AdminWebSiteController;

use actix_web::{dev::HttpServiceFactory, web};

/// 路由
pub struct AdminWebSiteRouter;

impl AdminWebSiteRouter {
    /// 注册`后台管理 WEB 服务`路由
    pub fn register() -> impl HttpServiceFactory {
        web::scope("")
            // .wrap(actix_web::middleware::NormalizePath::default())
            // 用于压缩响应有效负载的中间件。
            .wrap(actix_web::middleware::Compress::default())
            // 文件服务
            // .service(Files::new("/", "../web/dist").show_files_listing())
            // WEB 静态资源服务
            // .service(
            //     Files::new("/", "../web/dist")
            //         .index_file("index.html")
            //         .prefer_utf8(true),
            // )
            // WEB 静态资源服务
            .service(web::scope("/admin").route(
                "/{filename:.*}",
                web::get().to(AdminWebSiteController::index),
            ))
    }
}
