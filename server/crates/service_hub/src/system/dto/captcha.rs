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
    pub ids: Vec<i32>,
}

/// 添加验证码 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct AddCaptchaResp {
    pub captcha_id: String,
    pub base_img: String,
    pub expire: u32,
    pub created_at: DateTime<Local>,
}
