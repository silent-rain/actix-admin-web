//! 接口响应类型
use code::Error;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 数据列表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataList<T: Serialize> {
    data_list: Vec<T>,
    total: u64,
}

/// 数据类型
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Data<T: Serialize> {
    Json(Value),
    Data(T),
    DataList(DataList<T>),
}

/// 响应结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T: Serialize> {
    code: u16,
    msg: String,
    data: Data<T>,
}

#[allow(dead_code)]
impl<T: Serialize> Response<T> {
    /// 构建对象, 默认返回成功
    pub fn build() -> Self {
        Self {
            code: Error::OK.code(),
            msg: Error::OK.msg(),
            data: Data::Json(Value::Null),
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
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
    /// 设置返回的数据
    pub fn data(mut self, data: T) -> Self {
        self.data = Data::Data(data);
        self
    }
    /// 设置返回的 Json 类型的数据
    pub fn json_data(mut self, data: Value) -> Self {
        self.data = Data::Json(data);
        self
    }
    /// 设置返回的数据列表
    pub fn data_list(mut self, data_list: Vec<T>, total: u64) -> Self {
        self.data = Data::DataList(DataList { data_list, total });
        self
    }
}

/// 实现 actix_web 响应
impl<T: Serialize> Responder for Response<T> {
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
