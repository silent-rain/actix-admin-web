//! 打招呼
use serde::Deserialize;
use utoipa::IntoParams;

/// 打招呼 请求参数模型
#[derive(Default, Deserialize, IntoParams)]
pub struct GreetNameReq {
    pub name: String,
}
