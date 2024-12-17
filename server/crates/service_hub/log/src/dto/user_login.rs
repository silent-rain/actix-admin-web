//! 登陆日志管理
use entity::user::user_login_log;

use serde::{Deserialize, Serialize};

/// 查询登陆日志列表
#[derive(Default, Deserialize, Serialize)]
pub struct GetUserLoginListReq {
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
    /// 用户名称
    pub username: Option<String>,
}

/// 添加登陆日志信息
pub struct AddUserLoginInfoReq {
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 登陆令牌
    pub token: String,
    /// 登录IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 登录状态
    pub status: user_login_log::enums::Status,
}

/// 更新登陆日志信息
#[derive(Deserialize)]
pub struct UpdateUserLoginInfoReq {
    /// 描述信息
    pub desc: Option<String>,
    /// 登录状态
    pub status: user_login_log::enums::Status,
}

/// 更新登录日志状态
#[derive(Deserialize)]
pub struct UpdateUserLoginStatusReq {
    /// ID
    pub id: i32,
    /// 登录状态
    pub status: user_login_log::enums::Status,
}
