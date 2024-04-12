//! 登陆日志

use serde::Deserialize;

/// 查询登陆日志列表
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
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 登录IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 登录状态,0:失败,1:成功
    /// Enum: [`crate::log::enums::UserLoginStatus`]
    pub status: i8,
}

/// 更新登录日志状态
#[derive(Default, Deserialize)]
pub struct UpdateUserLoginStatusReq {
    /// ID
    pub id: i32,
    /// 登录状态,0:失败,1:成功
    /// Enum: [`crate::log::enums::UserLoginStatus`]
    pub status: i8,
}
