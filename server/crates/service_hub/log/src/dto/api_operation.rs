//! API操作日志

use entity::log_api_operation;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询API操作日志列表
#[derive(Default, Deserialize)]
pub struct GetApiOperationListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 添加API操作日志
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddApiOperationReq {
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 请求状态码
    pub status_code: i32,
    /// 请求方法
    pub method: String,
    /// 请求地址路径
    pub path: String,
    /// 请求参数
    pub query: Option<String>,
    /// 请求体/响应体
    pub body: Option<String>,
    /// 请求IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 耗时,纳秒
    pub cost: u64,
    /// 请求类型:REQ/RSP
    pub http_type: log_api_operation::enums::HttpType,
    /// 描述信息
    pub desc: Option<String>,
}
