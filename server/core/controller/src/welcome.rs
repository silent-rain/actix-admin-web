//!欢迎语
use dto::welcome::GreetNameReq;
use response::Response;

use tracing::warn;

/// 服务控制器层
pub struct Controller;

impl Controller {
    /// 打招呼
    pub async fn greet(params: GreetNameReq) -> Response {
        warn!("Hello, {}! You've been greeted from Rust!", params.name);
        Response::build().data(format!(
            "Hello, {}! You've been greeted from Rust!",
            params.name
        ))
    }
}
