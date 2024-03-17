//! 健康检查

use response::Response;

use actix_web::Responder;

/// 控制器
pub struct Controller;

impl Controller {
    /// 健康检查
    pub async fn health() -> impl Responder {
        Response::ok().data("ok")
    }
}
