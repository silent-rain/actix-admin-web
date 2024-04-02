//! 登陆日志

use serde::Deserialize;

/// 登陆日志列表查询
#[derive(Default, Deserialize)]
pub struct UserLoginListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 查询登陆日志信息
#[derive(Default, Deserialize)]
pub struct UserLoginInfoReq {
    pub id: i32,
}

/// 登陆状态
#[derive(Default, Deserialize)]
pub struct UserLoginStatusReq {
    pub id: i32,
    pub status: i8,
}
