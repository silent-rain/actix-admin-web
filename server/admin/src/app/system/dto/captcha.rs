//! 验证码

use actix_validator::Validate;

use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

/// 角色列表查询
#[derive(Default, Deserialize, Validate)]
pub struct CaptchaListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 通过 ID 查询验证码详情信息
#[derive(Default, Deserialize, Validate)]
pub struct CaptchaInfoReq {
    pub captcha_id: String,
}

/// 删除验证码
#[derive(Default, Deserialize, Validate)]
pub struct DeleteCaptchaReq {
    pub id: i32,
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
    pub expire: i8,
    pub created_at: DateTime,
}
