//! 角色管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 角色列表查询
#[derive(Default, Deserialize, Validate)]
pub struct RoleListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 添加角色
#[derive(Serialize, Deserialize, Validate)]
pub struct AddRoleReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    pub note: Option<String>,
}
