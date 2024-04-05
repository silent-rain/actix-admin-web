//! 登陆日志

use serde::Deserialize;

/// 登陆日志列表查询
#[derive(Default, Deserialize)]
pub struct UserLoginListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

/// 查询登陆日志信息
#[derive(Default, Deserialize)]
pub struct UserLoginInfoReq {
    pub id: i32,
}

/// 添加登陆日志信息
pub struct AddUserLoginInfoReq {
    pub user_id: i32,
    pub username: String,
    pub remote_addr: String,
    pub user_agent: String,
    pub status: i8,
}

/// 登陆状态
#[derive(Default, Deserialize)]
pub struct UserLoginStatusReq {
    pub id: i32,
    pub status: i8,
}
