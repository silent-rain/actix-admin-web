//! 用户角色关系管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户角色关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserRoleRelListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
}

/// 批量添加用户角色关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchAddUserRoleRelReq {
    pub user_id: i32,
    pub role_ids: Vec<i32>,
}

/// 批量删除用户角色关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteUserRoleRelReq {
    pub ids: Vec<i32>,
}
