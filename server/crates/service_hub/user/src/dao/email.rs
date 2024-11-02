//! 用户邮箱管理
use std::sync::Arc;

use crate::dto::email::GetEmailListReq;

use database::{Pagination, PoolTrait};
use entity::user::{user_email, UserEmail};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct EmailDao {
    db: Arc<dyn PoolTrait>,
}

impl EmailDao {
    /// 获取数据列表
    pub async fn list(&self, req: GetEmailListReq) -> Result<(Vec<user_email::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserEmail::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_email::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_email::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(user_email::Column::UserId.eq(v))
            })
            .apply_if(req.email, |query, v| {
                query.filter(user_email::Column::Email.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_email::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<user_email::Model>, DbErr> {
        UserEmail::find_by_id(id).one(self.db.db()).await
    }

    /// 通过邮箱获取详情信息
    pub async fn info_by_email(&self, email: String) -> Result<Option<user_email::Model>, DbErr> {
        UserEmail::find()
            .filter(user_email::Column::Email.eq(email))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: user_email::ActiveModel,
    ) -> Result<user_email::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: user_email::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = UserEmail::update_many()
            .set(active_model)
            .filter(user_email::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = UserEmail::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
