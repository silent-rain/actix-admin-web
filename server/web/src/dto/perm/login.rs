//! 登陆

use actix_validator::Validate;

use serde::Deserialize;

/// 通过 ID 查询用户详情信息
#[derive(Default, Deserialize, Validate)]
pub struct GetUserInfoReq {
    pub username: i32,
    pub password: i32,
}
