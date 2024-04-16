//! ICON图标

use entity::sys_icon;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 获取ICON图标列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetIconListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 图标名称
    pub name: Option<String>,
    /// 图标类型
    pub category: Option<i8>,
}

/// 获取ICON图标 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct GetIconRsp {
    #[serde(flatten)]
    pub dept: sys_icon::Model,
    /// Base64图片
    pub base_img: String,
}

/// 添加ICON图标
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct AddIconReq {
    /// 图标名称
    pub name: String,
    /// Base64图片
    pub base_img: String,
    /// 图标类型,1:element,2:custom
    pub category: i8,
    /// 备注
    pub note: Option<String>,
}

/// 更新ICON图标
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct UpdateIconReq {
    /// 图标ID
    pub id: i32,
    /// 图标名称
    pub name: String,
    /// Base64图片
    pub base_img: String,
    /// 图标类型,1:element,2:custom
    pub category: i8,
    /// 备注
    pub note: Option<String>,
}

/// 批量删除ICON图标
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteIconReq {
    /// ID列表
    pub ids: Vec<i32>,
}
