//! 健康检查

use response::Response;

use actix_web::Responder;
use tracing::error;

/// 控制器
pub struct HealthController;

impl HealthController {
    /// 健康检查
    pub async fn health() -> impl Responder {
        for i in 0..1000 {
            error!("this is test time zone {}", i);
        }
        Response::ok().data("ok")
    }
}
