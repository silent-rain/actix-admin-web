//! 系统日志

use serde::Deserialize;

/// 系统日志列表查询
#[derive(Default, Deserialize)]
pub struct GetSystemListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}
