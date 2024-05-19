//! 接口响应类型
use code::{Error, ErrorMsg};

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 数据列表
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DataList<T: Serialize> {
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
    /// 返回成功
    pub fn ok() -> Self {
        Self {
            code: Error::OK.code(),
            msg: Error::OK.msg(),
            data: None,
        }
    }
    /// 错误码
    pub fn code(code: Error) -> Self {
        Self {
            code: code.code(),
            msg: code.msg(),
            data: None,
        }
    }
    /// 通过错误码消息体创建响应体
    pub fn err(err: ErrorMsg) -> Self {
        Self {
            code: err.code(),
            msg: err.msg().to_owned(),
            data: None,
        }
    }
    /// 返回响应信息, 覆盖原响应信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }
    /// 追加响应信息, 在原响应信息的基础上添加新的信息
    pub fn append_msg(mut self, msg: &str) -> Self {
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

    /// 获取当前状态
    pub fn status(&self) -> u16 {
        self.code
    }
}

/// 打印 Response
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(status code: {}, msg: {})", self.code, self.msg)
    }
}

/// 错误码转换为响应体
impl From<Error> for Response {
    fn from(code: Error) -> Response {
        Response::code(code)
    }
}
/// 将错误信息转为响应体
impl From<ErrorMsg> for Response {
    fn from(err: ErrorMsg) -> Response {
        Response::err(err)
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

///  实现 actix_web 异常响应
impl ResponseError for Response {
    fn error_response(&self) -> HttpResponse {
        let body =
            serde_json::to_string(&self).unwrap_or_else(|e| format!("接口序列化异常: {:#?}", e));
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }
}
