/**提取器-路径
 *
 */
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct MyInfo {
    id: u32,
    username: String,
}

// 提取器可以作为处理程序函数的参数进行访问。
// Actix Web 支持每个处理程序函数最多 12 个提取器。论点位置无关紧要。
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

// 路径
// 路径提供从请求路径中提取的信息。
// 可提取的路径部分称为“动态段”，并用大括号标记。
// 您可以反序列化路径中的任何变量段。
/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index2(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

// 通过将动态段名称与字段名称匹配，将路径信息提取到实现特征的类型。
// 下面是一个使用元组类型的等效示例。
/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index3(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

// 作为非类型安全的替代方法，还可以在处理程序中按名称查询（请参阅match_info文档）路径参数的请求：
// https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html#method.match_info
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index4(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}
