//! 图片资源管理
use std::sync::Arc;

use crate::dto::image_resource::GetImageResourceListReq;

use database::{Pagination, PoolTrait};
use entity::{prelude::SysImageResource, sys_image_resource};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct ImageResourceDao {
    db: Arc<dyn PoolTrait>,
}

impl ImageResourceDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetImageResourceListReq,
    ) -> Result<(Vec<sys_image_resource::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysImageResource::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_image_resource::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_image_resource::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(sys_image_resource::Column::Name.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(sys_image_resource::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_image_resource::Model>, DbErr> {
        SysImageResource::find()
            .filter(sys_image_resource::Column::Id.eq(id))
            .one(self.db.db())
            .await
    }

    /// 通过hash值获取详情数据
    pub async fn info_by_hash(
        &self,
        hash: String,
    ) -> Result<Option<sys_image_resource::Model>, DbErr> {
        SysImageResource::find()
            .filter(sys_image_resource::Column::Hash.eq(hash))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_image_resource::ActiveModel,
    ) -> Result<sys_image_resource::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<sys_image_resource::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = SysImageResource::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 更新信息
    pub async fn update(
        &self,
        active_model: sys_image_resource::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = SysImageResource::update_many()
            .set(active_model)
            .filter(sys_image_resource::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysImageResource::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = SysImageResource::delete_many()
            .filter(sys_image_resource::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
