//! 用户邮箱管理

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询用户列表
#[derive(Default, Deserialize, Validate)]
pub struct GetEmailListReq {
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

/// 添加邮箱
#[derive(Serialize, Deserialize, Validate)]
pub struct AddEmailReq {
    /// 用户ID
    pub user_id: i32,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新邮箱
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateEmailReq {
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 描述信息
    pub desc: Option<String>,
}
