//! ICON图片
use crate::system::dto::icon::GetIconListReq;

use database::{DbRepo, Pagination};
use entity::{prelude::SysImage, sys_image};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct ImageDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> ImageDao<'a> {
    /// 获取数据列表
    pub async fn list(&self, req: GetIconListReq) -> Result<(Vec<sys_image::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysImage::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_image::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_image::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(sys_image::Column::Name.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(sys_image::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_image::Model>, DbErr> {
        SysImage::find()
            .filter(sys_image::Column::Id.eq(id))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_image::ActiveModel,
    ) -> Result<sys_image::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: sys_image::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = SysImage::update_many()
            .set(active_model)
            .filter(sys_image::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysImage::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = SysImage::delete_many()
            .filter(sys_image::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
