//! 角色管理
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 角色列表查询
#[derive(Default, Deserialize)]
pub struct GetRoleListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 通过 ID 查询角色详情信息
#[derive(Default, Deserialize)]
pub struct GetRoleInfoReq {
    pub id: i32,
}

/// 添加角色
#[derive(Serialize, Deserialize, Validate)]
pub struct AddRoleReq {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    pub note: Option<String>,
}

/// 删除角色
#[derive(Default, Deserialize)]
pub struct DeleteRoleReq {
    pub id: i32,
}
