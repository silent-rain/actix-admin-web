//! 欢迎语

use crate::dto::welcome::GreetNameReq;

use response::Response;

use actix_web::{web, Responder};
use tracing::info;

/// 控制器
pub struct Controller;

impl Controller {
    /// 打招呼
    pub async fn greet(req: web::Query<GreetNameReq>) -> impl Responder {
        info!("Hello, {}! You've been greeted from Rust!", req.name);
        Response::build().data(format!(
            "Hello, {}! You've been greeted from Rust!",
            req.name
        ))
    }
}
