//! 角色部门关系管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询角色部门关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetRoleDeptRelListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 角色ID
    pub role_id: Option<i32>,
}

/// 批量添加角色部门关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchAddRoleDeptRelReq {
    pub dept_id: i32,
    pub role_ids: Vec<i32>,
}

/// 批量删除角色部门关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteRoleDeptRelReq {
    pub ids: Vec<i32>,
}
