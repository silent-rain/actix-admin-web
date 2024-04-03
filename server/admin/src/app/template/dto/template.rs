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
}

/// 详情数据 请求体
#[derive(Default, Deserialize)]
pub struct AppTemplateInfoReq {
    pub id: i32,
}

/// 添加数据 请求体
#[derive(Default, Deserialize)]
pub struct AddAppTemplateStatusReq {
    pub user_id: String,
    pub status: i8,
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
