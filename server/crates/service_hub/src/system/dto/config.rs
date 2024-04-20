//! 配置管理

use crate::system::enums::ConfigStatus;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询配置列表
#[derive(Default, Deserialize, Validate)]
pub struct GetConfigListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 配置名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加配置
#[derive(Serialize, Deserialize, Validate)]
pub struct AddConfigReq {
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置编码(英文)
    pub code: String,
    /// 配置值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 配置描述
    pub desc: Option<String>,
    /// 状态, 0:停用,1:正常
    pub status: ConfigStatus,
}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateConfigReq {
    /// 配置ID
    pub id: i32,
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置编码(英文)
    pub code: String,
    /// 配置值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 配置描述
    pub desc: Option<String>,
    /// 状态,0:停用,1:正常
    pub status: ConfigStatus,
}

/// 更新数据状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateConfigStatusReq {
    /// 状态,0:停用,1:正常
    pub status: ConfigStatus,
}
