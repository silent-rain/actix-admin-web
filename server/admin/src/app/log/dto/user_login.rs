//! 登陆日志

use serde::Deserialize;

/// 登陆日志列表查询
#[derive(Default, Deserialize)]
pub struct GetUserLoginListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 添加登陆日志信息
pub struct AddUserLoginInfoReq {
    pub user_id: i32,
    pub username: String,
    pub remote_addr: String,
    pub user_agent: String,
    pub status: i8,
}

/// 更新登录日志状态
#[derive(Default, Deserialize)]
pub struct UpdateUserLoginStatusReq {
    pub id: i32,
    pub status: i8,
}
