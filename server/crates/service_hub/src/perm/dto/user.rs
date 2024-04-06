//! 用户管理
use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 用户列表查询
#[derive(Default, Deserialize, Validate)]
pub struct GetUserListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 添加用户
#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserReq {
    pub username: String,
    pub gender: i8,
    pub age: i32,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub avatar: Option<String>,
    pub role_ids: Vec<i32>,
}

/// 更新用户
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserReq {
    pub id: i32,
    pub username: String,
    pub gender: i8,
    pub age: i32,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub avatar: Option<String>,
    pub intro: Option<String>,
    pub note: Option<String>,
    pub status: i8,
    pub role_ids: Vec<i32>,
}

/// 获取用户个人信息
#[derive(Serialize, Deserialize)]
pub struct ProfileRsp {
    pub id: i32,
    pub username: Option<String>,
    pub gender: i8,
    pub age: Option<i32>,
    pub birthday: Option<String>,
    pub avatar: Option<String>,
}
