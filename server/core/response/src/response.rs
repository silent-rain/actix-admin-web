//! 接口响应类型
use code::Error;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 数据列表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataList<T: Serialize> {
    data_list: Vec<T>,
    total: u64,
}

/// 响应结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    /// 返回业务码
    code: u16,
    /// 返回信息
    msg: String,
    /// 返回数据
    data: Option<Value>,
}

#[allow(dead_code)]
impl Response {
    /// 构建对象, 默认返回成功
    pub fn build() -> Self {
        Self {
            code: Error::OK.code(),
            msg: Error::OK.msg(),
            data: None,
        }
    }
    /// 错误码
    pub fn code(mut self, code: Error) -> Self {
        self.code = code.code();
        self.msg = code.msg();
        self
    }
    /// 返回错误信息, 覆盖错误码信息
    pub fn msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }
    /// 追加错误信息, 在错误码信息的基础上添加新的信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
    /// 设置返回的数据
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        self.data = Some(json!(data));
        self
    }
    /// 设置返回的数据列表
    pub fn data_list<T: Serialize>(mut self, data_list: Vec<T>, total: u64) -> Self {
        self.data = Some(json!(DataList { data_list, total }));
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
