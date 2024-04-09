//! 后台管理 WEB 服务

use std::sync::Arc;

use app_state::AssetState;

use actix_web::{web::Data, HttpRequest, HttpResponse};
use tracing::warn;

/// 控制器
pub struct AdminWebSiteController;

impl AdminWebSiteController {
    /// 后台管理首页
    pub async fn index(
        req: HttpRequest,
        asset_state: Data<Arc<AssetState>>,
    ) -> Option<HttpResponse> {
        let mut filename = req.match_info().query("filename");
        if filename.is_empty() || filename == "/" {
            filename = "index.html"
        }
        warn!("req filename: {filename}");

        let asset = { asset_state.admin_web_dist.lock().unwrap().data(filename)? };
        let mimetype = {
            asset_state
                .admin_web_dist
                .lock()
                .unwrap()
                .mimetype(filename)?
        };

        // let asset = AssetWebDist::to_bytes(filename.to_string())?;
        // let mimetype = AssetWebDist::mimetype(filename.to_string())?;

        let content_type = format!("{mimetype}; charset=utf-8");
        let resp = HttpResponse::Ok()
            .insert_header(("Content-Type", content_type))
            .insert_header(("X-Hdr", "sample"))
            .body(asset);
        Some(resp)
    }
}
