//! 欢迎语
use actix_web::{get, web, Responder};
use dto::welcome::GreetNameReq;
use response::Response;

/// 打招呼
#[get("/greet")]
pub async fn greet(req: web::Query<GreetNameReq>) -> impl Responder {
    Response::build().data(format!(
        "Hello, {}! You've been greeted from Rust!",
        req.name
    ))
}
