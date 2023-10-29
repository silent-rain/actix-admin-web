//!通用模型
use serde::Deserialize;

/// 打招呼 请求参数模型
#[derive(Default, Deserialize)]
pub struct GreetNameReq {
    pub name: String,
}
