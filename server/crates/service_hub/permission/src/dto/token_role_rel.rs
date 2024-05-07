//! 令牌角色关系管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询令牌角色关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetTokenRoleRelListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 令牌ID
    pub token_id: Option<i32>,
}

/// 批量添加令牌角色关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchAddTokenRoleRelReq {
    /// 令牌ID
    pub token_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 批量删除令牌角色关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteTokenRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}
