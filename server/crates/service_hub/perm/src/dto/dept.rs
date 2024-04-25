//! 部门管理

use crate::enums::DeptStatus;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询部门列表
#[derive(Default, Deserialize, Validate)]
pub struct GetDeptListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 角色名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加部门
#[derive(Serialize, Deserialize, Validate)]
pub struct AddDeptReq {
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDeptReq {
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: DeptStatus,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDeptStatusReq {
    /// 状态,0:停用,1:正常
    pub status: DeptStatus,
}
