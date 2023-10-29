use std::cell::Cell;

/**提取器-应用程序状态提取器
 *
 */
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}
