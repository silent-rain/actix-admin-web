/**提取器--路径
 *
 */
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

// Query<T> 类型为请求的查询参数提供提取功能。它下面使用板条箱。
#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

// 网址编码表单
/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
#[post("/")]
async fn index2(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}
