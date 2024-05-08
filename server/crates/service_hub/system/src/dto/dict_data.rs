//! 字典数据管理

use entity::sys_dict_data;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询字典数据列表
#[derive(Default, Deserialize, Validate)]
pub struct GetDictDataListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 字典标签
    pub lable: Option<String>,
    /// 字典维度ID
    pub dim_id: Option<i32>,
    /// 字典维度编码
    pub dim_code: Option<String>,
}

/// 添加字典数据
#[derive(Serialize, Deserialize, Validate)]
pub struct AddDictDataReq {
    /// 字典维度ID
    pub dim_id: i32,
    /// 字典维度编码
    pub dim_code: String,
    /// 字典标签
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub lable: String,
    /// 字典键值
    pub value: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataReq {
    /// 字典标签
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub lable: String,
    /// 字典键值
    pub value: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: sys_dict_data::enums::Status,
}

/// 更新字典数据状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: sys_dict_data::enums::Status,
}
