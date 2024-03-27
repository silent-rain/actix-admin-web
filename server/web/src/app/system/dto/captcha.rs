//! 验证码

use actix_validator::Validate;

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
    pub id: i32,
}

/// 添加验证码
#[derive(Serialize, Deserialize, Validate)]
pub struct AddCaptchaReq {
    pub uuid: String,
    pub captcha: String,
    pub base_img: Vec<u8>,
    pub expire: i8,
}

/// 删除验证码
#[derive(Default, Deserialize, Validate)]
pub struct DeleteCaptchaReq {
    pub id: i32,
}
