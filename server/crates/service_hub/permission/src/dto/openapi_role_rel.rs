//! OpenApi接口角色关系管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询OpenApi接口角色关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetOpenapiRoleRelListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 接口ID
    pub openapi_id: Option<i32>,
}

/// 批量添加OpenApi接口角色关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchAddOpenapiRoleRelReq {
    /// 接口ID
    pub openapi_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 批量删除OpenApi接口角色关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteOpenapiRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}
