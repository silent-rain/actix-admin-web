//! 欢迎语
use actix_web::{web, Responder};
use dto::welcome::GreetNameReq;
use response::Response;

/// 路由层
pub struct Routes;

impl Routes {
    /// 打招呼
    pub async fn greet(req: web::Query<GreetNameReq>) -> impl Responder {
        Response::build().data(format!(
            "Hello, {}! You've been greeted from Rust!",
            req.name
        ))
    }
}
