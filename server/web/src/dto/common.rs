//!通用模型
use serde::Deserialize;

/// 通过 ID 查询信息
#[derive(Default, Deserialize)]
pub struct QueryIdReq {
    pub id: i32,
}
