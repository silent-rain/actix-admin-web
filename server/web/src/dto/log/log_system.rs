//! 系统日志
use serde::Deserialize;

/// 系统日志列表查询
#[derive(Default, Deserialize)]
pub struct LogSystemListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
}

/// 查询系统日志信息
#[derive(Default, Deserialize)]
pub struct LogSystemInfoReq {
    pub id: i32,
}

/// 删除系统日志信息
#[derive(Default, Deserialize)]
pub struct DeleteLogSystemReq {
    pub id: i32,
}
