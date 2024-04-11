//! 角色管理
use crate::perm::enums::RoleStatus;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询角色列表
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
    /// 角色名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
}

/// 更新数据 请求体
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRoleReq {
    /// 角色ID
    pub id: i32,
    /// 角色名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: RoleStatus,
}

/// 更新数据状态
#[derive(Default, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRoleStatusReq {
    /// 角色ID
    pub id: i32,
    /// 状态,0:停用,1:正常
    pub status: RoleStatus,
}
