//! 模板管理

use crate::template::enums::AppTemplateStatus;

use actix_validator::Validate;

use serde::Deserialize;

/// 查询列表数据 请求体
#[derive(Default, Deserialize)]
pub struct GetAppTemplateListReq {
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

/// 添加数据 请求体
#[derive(Deserialize)]
pub struct AddAppTemplateReq {
    /// 用户ID
    pub user_id: i32,
    /// 状态,0:停用,1:正常
    pub status: AppTemplateStatus,
}

/// 批量添加数据结点
#[derive(Deserialize)]
pub struct BatchAddAppTemplateNode {
    /// 用户ID
    pub user_id: i32,
    /// 状态,0:停用,1:正常
    pub status: AppTemplateStatus,
}

/// 批量添加数据 请求体
#[derive(Default, Deserialize)]
pub struct BatchAddAppTemplateReq {
    /// 数据列表
    pub data: Vec<BatchAddAppTemplateNode>,
}

/// 更新数据 请求体
#[derive(Deserialize)]
pub struct UpdateAppTemplateReq {
    /// 状态,0:停用,1:正常
    pub status: AppTemplateStatus,
}

/// 更新数据状态 请求体
#[derive(Deserialize)]
pub struct UpdateAppTemplateStatusReq {
    /// 状态,0:停用,1:正常
    pub status: AppTemplateStatus,
}

/// 批量删除数据 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteAppTemplateReq {
    /// ID列表
    pub ids: Vec<i32>,
}
