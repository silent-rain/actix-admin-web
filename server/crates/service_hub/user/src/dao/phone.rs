//! 用户手机号管理
use std::sync::Arc;

use crate::dto::phone::GetPhoneListReq;

use database::{Pagination, PoolTrait};
use entity::user::{phone, Phone};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct PhoneDao {
    db: Arc<dyn PoolTrait>,
}

impl PhoneDao {
    /// 获取数据列表
    pub async fn list(&self, req: GetPhoneListReq) -> Result<(Vec<phone::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = Phone::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(phone::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(phone::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(phone::Column::UserId.eq(v))
            })
            .apply_if(req.phone, |query, v| {
                query.filter(phone::Column::Phone.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(phone::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<phone::Model>, DbErr> {
        Phone::find_by_id(id).one(self.db.db()).await
    }

    /// 通过手机号码获取详情信息
    pub async fn info_by_phone(&self, phone: String) -> Result<Option<phone::Model>, DbErr> {
        Phone::find()
            .filter(phone::Column::Phone.eq(phone))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: phone::ActiveModel) -> Result<phone::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: phone::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = Phone::update_many()
            .set(active_model)
            .filter(phone::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = Phone::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
