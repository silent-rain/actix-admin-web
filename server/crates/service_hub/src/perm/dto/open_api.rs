//! OpenApi接口管理

use crate::perm::enums::{OpenApiCategory, OpenApiStatus};

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询OpenApi接口列表
#[derive(Clone, Deserialize, Validate)]
pub struct GetOpenApiListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 接口名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加OpenApi接口
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct AddOpenApiReq {
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: OpenApiCategory,
    /// 接口名称
    pub name: String,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态, 0:停用,1:正常
    pub status: OpenApiStatus,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateOpenApiReq {
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: OpenApiCategory,
    /// 接口名称
    pub name: String,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
    /// 排序
    pub sort: Option<i32>,
    /// 备注
    pub note: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: OpenApiStatus,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateOpenApiStatusReq {
    /// 状态,0:停用,1:正常
    pub status: OpenApiStatus,
}
