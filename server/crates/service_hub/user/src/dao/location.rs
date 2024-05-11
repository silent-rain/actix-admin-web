//! 用户地理位置管理
use crate::dto::location::GetLocationListReq;

use database::{DbRepo, Pagination};
use entity::user::{location, Location};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct LocationDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> LocationDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetLocationListReq,
    ) -> Result<(Vec<location::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = Location::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(location::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(location::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(location::Column::UserId.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(location::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<location::Model>, DbErr> {
        Location::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过用户ID获取详情信息
    pub async fn info_user_id(&self, user_id: i32) -> Result<Option<location::Model>, DbErr> {
        Location::find()
            .filter(location::Column::UserId.eq(user_id))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: location::ActiveModel) -> Result<location::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: location::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = Location::update_many()
            .set(active_model)
            .filter(location::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = Location::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
