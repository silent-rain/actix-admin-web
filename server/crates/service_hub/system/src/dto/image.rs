//! 图片

use actix_validator::Validate;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use serde::{Deserialize, Serialize};

/// 获取图片列表 请求体
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

/// 多文件上传
#[derive(Debug, MultipartForm)]
pub struct UploadFilesForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

/// 单文件上传
#[derive(Debug, MultipartForm)]
pub struct UploadFileForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
}

/// 更新图片
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct UpdateIconReq {
    /// 图片名称
    pub name: String,
    /// 描述信息
    pub desc: Option<String>,
}

/// 批量删除图片
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteIconReq {
    /// ID列表
    pub ids: Vec<i32>,
}
