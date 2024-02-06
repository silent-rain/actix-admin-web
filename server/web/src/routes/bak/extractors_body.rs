/**提取器-Json
 *
 */
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// 允许将请求正文反序列化为结构。
/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

