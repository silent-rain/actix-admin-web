//! 接口响应类型
use code::Error;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 数据列表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataList<T: Serialize> {
    data_list: T,
    total: u64,
}

/// 响应结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    code: u16,
    msg: String,
    data: Value,
}

#[allow(dead_code)]
impl Response {
    pub fn new(code: u16, msg: &str) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            data: Value::Null,
        }
    }
    pub fn build() -> Self {
        Self {
            code: Error::OK.code(),
            msg: "".to_string(),
            data: Value::Null,
        }
    }
    /// 错误码
    pub fn code(mut self, code: Error) -> Self {
        self.code = code.code();
        self.msg = code.msg();
        self
    }
    /// 返回错误信息, 覆盖原始错误码信息
    pub fn msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }
    /// 追加错误信息, 保留原始错误码信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
    /// Set the data of the `Response` to `data`.
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        self.data = json!(data);
        self
    }
    /// Set the data of the `Response` to `data`.
    pub fn data2(mut self, data: Value) -> Self {
        self.data = data;
        self
    }
    /// Set the data of the `Response` to `data list`.
    pub fn data_list<T: Serialize>(mut self, data_list: &[T], total: u64) -> Self {
        self.data = json!(DataList { data_list, total });
        self
    }
}

/// 实现 actix_web 响应
impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body =
            serde_json::to_string(&self).unwrap_or_else(|e| format!("接口序列化异常: {:#?}", e));

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
