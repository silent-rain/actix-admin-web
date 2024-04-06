//! 后台管理 WEB 服务

use crate::asset::AssetWebDist;

use actix_web::{dev::HttpServiceFactory, web, HttpRequest, HttpResponse};

/// 控制器
struct AdminWebSiteController;

impl AdminWebSiteController {
    /// 后台管理首页
    pub async fn index(req: HttpRequest) -> Option<HttpResponse> {
        let mut filename = req.match_info().query("filename");
        if filename.is_empty() || filename == "/" {
            filename = "index.html"
        }

        let asset = AssetWebDist::to_bytes(filename.to_string())?;
        let mimetype = AssetWebDist::mimetype(filename.to_string())?;

        let content_type = format!("{mimetype}; charset=utf-8");
        let resp = HttpResponse::Ok()
            .insert_header(("Content-Type", content_type))
            .insert_header(("X-Hdr", "sample"))
            .body(asset);
        Some(resp)
    }
}

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
