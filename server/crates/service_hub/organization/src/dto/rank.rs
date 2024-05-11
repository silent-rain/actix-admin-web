//! 职级管理

use entity::organization::rank;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询职级列表
#[derive(Default, Deserialize, Validate)]
pub struct GetRankListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 职级名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加职级
#[derive(Serialize, Deserialize, Validate)]
pub struct AddRankReq {
    /// 职级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///职级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: rank::enums::Status,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRankReq {
    /// 职级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///职级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: rank::enums::Status,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRankStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: rank::enums::Status,
}
