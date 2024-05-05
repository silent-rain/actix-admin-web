//! 用户邮箱管理

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserEmailListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 邮箱
    pub email: Option<String>,
}

/// 添加用户手机号
#[derive(Serialize, Deserialize, Validate)]
pub struct AddUserEmailReq {
    /// 用户ID
    pub user_id: i32,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 备注
    pub note: Option<String>,
}

/// 更新用户手机号
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateUserEmailReq {
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 备注
    pub note: Option<String>,
}
