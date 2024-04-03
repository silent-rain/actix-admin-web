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
