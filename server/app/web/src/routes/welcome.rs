//! 欢迎语
use dto::welcome::GreetNameReq;
use response::Response;

use actix_web::{web, Responder, Scope};
use tracing::info;

/// 打招呼
pub fn register() -> Scope {
    web::scope("/greet").route("", web::get().to(Routes::greet))
}

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
