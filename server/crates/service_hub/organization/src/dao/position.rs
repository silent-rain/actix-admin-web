//! 岗位管理
use std::sync::Arc;

use crate::dto::position::GetPositionListReq;

use database::{Pagination, PoolTrait};
use entity::organization::{position, Position};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct PositionDao {
    db: Arc<dyn PoolTrait>,
}

impl PositionDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<position::Model>, u64), DbErr> {
        let results = Position::find()
            .order_by_asc(position::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetPositionListReq,
    ) -> Result<(Vec<position::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = Position::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(position::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(position::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(position::Column::Name.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(position::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<position::Model>, DbErr> {
        Position::find_by_id(id).one(self.db.db()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(&self, name: String) -> Result<Option<position::Model>, DbErr> {
        Position::find()
            .filter(position::Column::Name.eq(name))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: position::ActiveModel) -> Result<position::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: position::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = Position::update_many()
            .set(active_model)
            .filter(position::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = position::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = Position::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
