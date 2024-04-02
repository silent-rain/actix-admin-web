//! 模板管理

use crate::app::template::dto::template::AppTemplateListReq;

use database::{DbRepo, Pagination};
use entity::{app_template, prelude::AppTemplate};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[injectable]
pub struct AppTemplateDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> AppTemplateDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let results = AppTemplate::find()
            .order_by_asc(app_template::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取列表数据
    pub async fn list(
        &self,
        req: AppTemplateListReq,
    ) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = AppTemplate::find()
            .order_by_desc(app_template::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, num_pages))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<app_template::Model>, DbErr> {
        AppTemplate::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: app_template::ActiveModel,
    ) -> Result<app_template::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: app_template::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = AppTemplate::update_many()
            .set(active_model)
            .filter(app_template::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = app_template::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = AppTemplate::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = AppTemplate::delete_many()
            .filter(app_template::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
