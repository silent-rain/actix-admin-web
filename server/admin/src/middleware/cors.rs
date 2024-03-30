//! 跨域中间件

use actix_cors::Cors;

// 跨域中间件包装
pub fn wrap_cors() -> Cors {
    Cors::default()
        // .allowed_origin("https://www.rust-lang.org")
        // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
        // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        // .allowed_header(http::header::CONTENT_TYPE)
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
