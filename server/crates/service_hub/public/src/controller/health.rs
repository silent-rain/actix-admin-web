//! 健康检查

use response::Response;

use actix_web::Responder;

/// 控制器
pub struct HealthController;

impl HealthController {
    /// 健康检查
    pub async fn health() -> impl Responder {
        Response::ok().data("ok")
    }
}
