//! 用户邮箱管理
use crate::dto::user_email::GetUserEmailListReq;

use database::{DbRepo, Pagination};
use entity::perm_user_email;
use entity::prelude::PermUserEmail;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct UserEmailDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserEmailDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserEmailListReq,
    ) -> Result<(Vec<perm_user_email::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermUserEmail::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_user_email::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_user_email::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(perm_user_email::Column::UserId.eq(v))
            })
            .apply_if(req.email, |query, v| {
                query.filter(perm_user_email::Column::Email.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_user_email::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user_email::Model>, DbErr> {
        PermUserEmail::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过邮箱获取详情信息
    pub async fn info_by_email(
        &self,
        email: String,
    ) -> Result<Option<perm_user_email::Model>, DbErr> {
        PermUserEmail::find()
            .filter(perm_user_email::Column::Email.eq(email))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_user_email::ActiveModel,
    ) -> Result<perm_user_email::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: perm_user_email::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermUserEmail::update_many()
            .set(active_model)
            .filter(perm_user_email::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUserEmail::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
