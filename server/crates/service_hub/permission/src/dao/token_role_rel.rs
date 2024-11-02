//! 令牌角色关系管理

use std::sync::Arc;

use crate::dto::token_role_rel::GetTokenRoleRelListReq;

use database::{Pagination, PoolTrait};
use entity::{perm_token_role_rel, prelude::PermTokenRoleRel};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct TokenRoleRelDao {
    db: Arc<dyn PoolTrait>,
}

impl TokenRoleRelDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetTokenRoleRelListReq,
    ) -> Result<(Vec<perm_token_role_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermTokenRoleRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_token_role_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_token_role_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.token_id, |query, v| {
                query.filter(perm_token_role_rel::Column::TokenId.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_token_role_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list_by_token_id(
        &self,
        token_id: i32,
    ) -> Result<(Vec<perm_token_role_rel::Model>, u64), DbErr> {
        let results = PermTokenRoleRel::find()
            .filter(perm_token_role_rel::Column::TokenId.eq(token_id))
            .all(self.db.db())
            .await?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: perm_token_role_rel::ActiveModel,
    ) -> Result<perm_token_role_rel::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<perm_token_role_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = PermTokenRoleRel::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermTokenRoleRel::delete_many()
            .filter(perm_token_role_rel::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = PermTokenRoleRel::delete_many()
            .filter(perm_token_role_rel::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
