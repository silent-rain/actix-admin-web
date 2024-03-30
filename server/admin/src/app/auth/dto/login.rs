//! 登陆

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 登陆 请求体
#[derive(Default, Clone, Deserialize, Validate)]
pub struct LoginReq {
    pub username: String, // 手机号码或邮箱
    pub password: String, // 登陆密码
    pub captcha_id: String,
    pub captcha: String,
}

/// 登陆 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct LoginRsp {
    pub user_id: i32,
    pub token: String,
}
