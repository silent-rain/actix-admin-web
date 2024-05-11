//! 岗位管理

use entity::organization::position;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询岗位列表
#[derive(Default, Deserialize, Validate)]
pub struct GetPositionListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 岗位名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加岗位
#[derive(Serialize, Deserialize, Validate)]
pub struct AddPositionReq {
    /// 岗位名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 状态(0:停用,1:正常)
    pub status: position::enums::Status,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePositionReq {
    /// 岗位名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 状态(0:停用,1:正常)
    pub status: position::enums::Status,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePositionStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: position::enums::Status,
}
