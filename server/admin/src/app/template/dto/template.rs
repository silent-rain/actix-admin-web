//! 模板管理

use actix_validator::Validate;

use serde::Deserialize;

/// 查询列表数据 请求体
#[derive(Default, Deserialize)]
pub struct AppTemplateListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 排序字段
    pub order_by: Option<String>,
}

/// 详情数据 请求体
#[derive(Default, Deserialize)]
pub struct AppTemplateInfoReq {
    pub id: i32,
}

/// 添加数据 请求体
#[derive(Default, Deserialize)]
pub struct AddAppTemplateReq {
    pub user_id: i32,
    pub status: i8,
}

/// 数据
#[derive(Default, Deserialize)]
pub struct AppTemplate {
    pub user_id: i32,
    pub status: i8,
}

/// 批量添加数据 请求体
#[derive(Default, Deserialize)]
pub struct BatchAddAppTemplateReq {
    pub data: Vec<AppTemplate>,
}

/// 更新数据 请求体
#[derive(Default, Deserialize)]
pub struct UpdateAppTemplateReq {
    pub id: i32,
    pub status: i8,
}

/// 更新数据状态 请求体
#[derive(Default, Deserialize)]
pub struct UpdateAppTemplateStatusReq {
    pub id: i32,
    pub status: i8,
}

/// 批量删除数据 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteAppTemplateReq {
    pub ids: Vec<i32>,
}
