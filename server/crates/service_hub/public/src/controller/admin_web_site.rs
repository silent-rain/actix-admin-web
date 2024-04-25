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
        let r = asset_state.admin_web_dist.read().await;
        let asset = r.data(filename)?;
        let mimetype = r.mimetype(filename)?;

        let content_type = format!("{mimetype}; charset=utf-8");
        let resp = HttpResponse::Ok()
            .insert_header(("Content-Type", content_type))
            .insert_header(("X-Hdr", "sample"))
            .body(asset);
        Some(resp)
    }
}
