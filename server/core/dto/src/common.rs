//!通用模型
use serde::Deserialize;

/// 通用 ID 请求参数模型
#[derive(Default, Deserialize)]
pub struct QueryIdReq {
    pub id: i32,
}
