//! 欢迎语
use actix_web::{get, web, Responder};
use dto::welcome::GreetNameReq;
use response::Response;

/// 打招呼
#[utoipa::path(
    get,
    path = "/api/v1/greet",
    params(
        GreetNameReq,
    ),
    responses(
        (status = 200, description = "succesfully", body=Response),
        (status = NOT_FOUND, description = "not found")
    ),
    security(
        ("api_key" = [])
    ),
)]
#[get("/greet")]
pub async fn greet(req: web::Query<GreetNameReq>) -> impl Responder {
    Response::build().data(format!(
        "Hello, {}! You've been greeted from Rust!",
        req.name
    ))
}
