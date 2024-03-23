//! 系统日志

use crate::app::log::dto::system::LogSystemListReq;

use database::DBRepo;
use entity::log::system;
use entity::prelude::LogSystem;
use utils::pagination::Pagination;

use nject::injectable;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DbErr};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

#[injectable]
pub struct LogSystemDao<'a> {
    db: &'a dyn DBRepo,
}

impl<'a> LogSystemDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<Vec<system::Model>, DbErr> {
        let result = LogSystem::find()
            .order_by_desc(system::Column::Id)
            .all(self.db.rdb())
            .await?;
        Ok(result)
    }

    /// 获取数据列表
    pub async fn list(&self, req: LogSystemListReq) -> Result<(Vec<system::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = LogSystem::find()
            .order_by_desc(system::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<system::Model>, DbErr> {
        LogSystem::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(&self, data: system::Model) -> Result<system::Model, DbErr> {
        let mut active_model: system::ActiveModel = data.into();
        active_model.id = NotSet;
        active_model.insert(self.db.wdb()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = LogSystem::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
