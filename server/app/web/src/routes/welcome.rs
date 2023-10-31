//! 欢迎语
use dto::welcome::GreetNameReq;
use response::Response;

use actix_web::{web, Responder};
use tracing::info;

/// 路由层
pub struct Routes;

impl Routes {
    /// 打招呼
    pub async fn greet(req: web::Query<GreetNameReq>) -> impl Responder {
        info!("Hello, {}! You've been greeted from Rust!", req.name);
        Response::build().data(format!(
            "Hello, {}! You've been greeted from Rust!",
            req.name
        ))
    }
}
