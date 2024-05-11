//! 会员等级管理

use entity::user::member_level;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询会员等级列表
#[derive(Default, Deserialize, Validate)]
pub struct GetMemberLevelListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 会员等级名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加会员等级
#[derive(Serialize, Deserialize, Validate)]
pub struct AddMemberLevelReq {
    /// 会员等级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///会员等级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: member_level::enums::Status,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberLevelReq {
    /// 会员等级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///会员等级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: member_level::enums::Status,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberLevelStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: member_level::enums::Status,
}
