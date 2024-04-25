//! 用户Token令牌与角色关系管理

use crate::dto::user_token_role_rel::GetUserTokenRoleRelListReq;

use database::{DbRepo, Pagination};
use entity::{perm_user_token_role_rel, prelude::PermUserTokenRoleRel};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct UserTokenRoleRelDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserTokenRoleRelDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserTokenRoleRelListReq,
    ) -> Result<(Vec<perm_user_token_role_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermUserTokenRoleRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_user_token_role_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_user_token_role_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.token_id, |query, v| {
                query.filter(perm_user_token_role_rel::Column::TokenId.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_user_token_role_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: perm_user_token_role_rel::ActiveModel,
    ) -> Result<perm_user_token_role_rel::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<perm_user_token_role_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = PermUserTokenRoleRel::insert_many(active_models)
            .exec(self.db.wdb())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUserTokenRoleRel::delete_many()
            .filter(perm_user_token_role_rel::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = PermUserTokenRoleRel::delete_many()
            .filter(perm_user_token_role_rel::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
