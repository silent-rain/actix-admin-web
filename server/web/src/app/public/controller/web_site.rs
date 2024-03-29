//! 静态资源
use crate::asset::AssetWebDist;

use actix_web::{HttpRequest, HttpResponse};

/// 控制器
pub struct WebSiteController;

impl WebSiteController {
    /// 首页
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
