//! 系统日志

use database::DBRepo;
use dto::log::system::SystemLogListReq;
use dto::pagination::Pagination;
use entity::log_system;
use entity::prelude::LogSystem;

use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DbErr};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

pub struct Dao<'a, DB: DBRepo> {
    db: &'a DB,
}

impl<'a, DB: DBRepo> Dao<'a, DB> {
    /// 创建对象
    pub fn new(db: &'a DB) -> Self {
        Dao { db }
    }

    /// 获取所有数据
    pub async fn all(&self) -> Result<Vec<log_system::Model>, DbErr> {
        let result = LogSystem::find()
            .order_by_desc(log_system::Column::Id)
            .all(self.db.rdb())
            .await?;
        Ok(result)
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: SystemLogListReq,
    ) -> Result<(Vec<log_system::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = LogSystem::find()
            .order_by_desc(log_system::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<log_system::Model>, DbErr> {
        LogSystem::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(&self, data: log_system::Model) -> Result<log_system::Model, DbErr> {
        let mut active_model: log_system::ActiveModel = data.into();
        active_model.id = NotSet;
        active_model.insert(self.db.wdb()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = LogSystem::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
