//! 用户邮箱管理
use std::sync::Arc;

use crate::dto::email::GetEmailListReq;

use database::{Pagination, PoolTrait};
use entity::user::{email, Email};

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
    pub async fn list(&self, req: GetEmailListReq) -> Result<(Vec<email::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = Email::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(email::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(email::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(email::Column::UserId.eq(v))
            })
            .apply_if(req.email, |query, v| {
                query.filter(email::Column::Email.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(email::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<email::Model>, DbErr> {
        Email::find_by_id(id).one(self.db.db()).await
    }

    /// 通过邮箱获取详情信息
    pub async fn info_by_email(&self, email: String) -> Result<Option<email::Model>, DbErr> {
        Email::find()
            .filter(email::Column::Email.eq(email))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: email::ActiveModel) -> Result<email::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: email::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = Email::update_many()
            .set(active_model)
            .filter(email::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = Email::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
