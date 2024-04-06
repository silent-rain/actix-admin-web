//! 角色管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 角色列表查询
#[derive(Default, Deserialize, Validate)]
pub struct GetRoleListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加角色
#[derive(Serialize, Deserialize, Validate)]
pub struct AddRoleReq {
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    pub sort: i32,
    pub note: Option<String>,
    pub status: i8,
}

/// 更新数据 请求体
#[derive(Default, Serialize, Deserialize, Validate)]
pub struct UpdateRoleReq {
    pub id: i32,
    pub status: i8,
    pub name: String,
    pub note: Option<String>,
    pub sort: i32,
}
