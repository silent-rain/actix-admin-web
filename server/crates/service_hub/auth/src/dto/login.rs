//! 登陆

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 登陆 请求体
#[derive(Default, Clone, Deserialize, Validate)]
pub struct LoginReq {
    /// 手机号码或邮箱
    pub username: String,
    /// 登陆密码
    pub password: String,
    /// 验证码ID
    pub captcha_id: String,
    /// 验证码
    pub captcha: String,
}

/// 登陆 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct LoginRsp {
    /// 用户ID
    pub user_id: i32,
    /// Token 令牌
    pub token: String,
}

/// 浏览器信息
#[derive(Default, Deserialize, Serialize)]
pub struct BrowserInfo {
    /// Peer socket address.
    pub remote_addr: String,
    /// User Agent
    pub user_agent: String,
}
