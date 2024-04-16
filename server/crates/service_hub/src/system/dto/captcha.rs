//! 验证码

use actix_validator::Validate;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// 获取验证码列表
#[derive(Default, Deserialize, Validate)]
pub struct GetCaptchaListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 批量删除验证码
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteCaptchaReq {
    /// ID列表
    pub ids: Vec<i32>,
}

/// 添加验证码 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct AddCaptchaResp {
    /// 验证码ID
    pub captcha_id: String,
    /// Base64图片
    pub base_img: String,
    /// 过期时间,秒
    pub expire: u32,
    /// 创建时间
    pub created_at: DateTime<Local>,
}
