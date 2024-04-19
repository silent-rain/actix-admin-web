// 分页器
use serde::Deserialize;

/// 分页器
#[derive(Default, Deserialize)]
pub struct Pagination {
    /// 当前分页
    page: Option<u64>,
    /// 页面大小
    page_size: Option<u64>,
}

#[allow(dead_code)]
impl Pagination {
    /// 创建分页器对象
    pub fn new(page: u64, page_size: u64) -> Self {
        Self {
            page: Some(page),
            page_size: Some(page_size),
        }
    }
    /// 页数
    pub fn page(&self) -> u64 {
        let page = self.page.map_or(1, |v| v);
        if page == 0 {
            return 0;
        }
        page - 1
    }
    /// 页面数据大小, 默认 100 条数据
    pub fn page_size(&self) -> u64 {
        self.page_size.map_or(100, |v| v)
    }
    /// 偏移大小
    pub fn offset(&self) -> u64 {
        self.page() * self.page_size()
    }
}
