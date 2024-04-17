//! ICON图片

use entity::sys_icon;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 获取ICON图片列表 请求体
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
    /// 图片名称
    pub name: Option<String>,
}

/// 获取ICON图片 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct GetIconRsp {
    #[serde(flatten)]
    pub data: sys_icon::Model,
    /// Base64图片
    pub base_img: String,
}

/// 添加ICON图片
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct AddIconReq {
    /// 图片名称
    pub name: String,
    /// HASH名称
    pub hash_name: String,
    /// Base64图片
    pub base_img: String,
    /// 扩展类型,svg,png
    /// Enum: [`crate::system::enums::IconType`]
    pub icon_type: String,
    /// 备注
    pub note: Option<String>,
}

/// 更新ICON图片
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct UpdateIconReq {
    /// 图片名称
    pub name: String,
    /// HASH名称
    pub hash_name: String,
    /// Base64图片
    pub base_img: String,
    /// 扩展类型,svg,png
    /// Enum: [`crate::system::enums::IconType`]
    pub icon_type: String,
    /// 备注
    pub note: Option<String>,
}

/// 批量删除ICON图片
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteIconReq {
    /// ID列表
    pub ids: Vec<i32>,
}
