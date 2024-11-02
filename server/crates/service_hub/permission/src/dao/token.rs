//! 令牌管理
use std::sync::Arc;

use crate::dto::token::GetTokenListReq;

use database::{Pagination, PoolTrait};
use entity::{perm_token, prelude::PermToken};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct TokenDao {
    db: Arc<dyn PoolTrait>,
}

impl TokenDao {
    /// 获取数据列表
    pub async fn list(&self, req: GetTokenListReq) -> Result<(Vec<perm_token::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermToken::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_token::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_token::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(perm_token::Column::UserId.eq(v))
            })
            .apply_if(req.token, |query, v| {
                query.filter(perm_token::Column::Token.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_token::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_token::Model>, DbErr> {
        PermToken::find_by_id(id).one(self.db.db()).await
    }

    /// 通过Token获取详情信息
    pub async fn info_by_token(
        &self,
        token: String,
        passphrase: String,
    ) -> Result<Option<perm_token::Model>, DbErr> {
        PermToken::find()
            .filter(perm_token::Column::Token.eq(token))
            .filter(perm_token::Column::Passphrase.eq(passphrase))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_token::ActiveModel,
    ) -> Result<perm_token::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: perm_token::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermToken::update_many()
            .set(active_model)
            .filter(perm_token::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = perm_token::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermToken::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
