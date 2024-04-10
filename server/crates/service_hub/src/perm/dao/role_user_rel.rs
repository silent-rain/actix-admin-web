//! 角色与用户关系管理

use crate::perm::dto::user_role_rel::GetUserRoleRelListReq;

use database::{DbRepo, Pagination};
use entity::{perm_role_user_rel, prelude::PermRoleUserRel};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct UserRoleRelDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserRoleRelDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserRoleRelListReq,
    ) -> Result<(Vec<perm_role_user_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermRoleUserRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_role_user_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_role_user_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(perm_role_user_rel::Column::UserId.like(format!("%{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;

        let results = states
            .order_by_desc(perm_role_user_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 添加关联关系
    pub async fn add(
        &self,
        active_model: perm_role_user_rel::ActiveModel,
    ) -> Result<perm_role_user_rel::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<perm_role_user_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = PermRoleUserRel::insert_many(active_models)
            .exec(self.db.wdb())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermRoleUserRel::delete_many()
            .filter(perm_role_user_rel::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = PermRoleUserRel::delete_many()
            .filter(perm_role_user_rel::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}

impl<'a> UserRoleRelDao<'a> {
    /// 获取数据列表
    pub async fn list_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<(Vec<perm_role_user_rel::Model>, u64), DbErr> {
        let results = PermRoleUserRel::find()
            .filter(perm_role_user_rel::Column::UserId.eq(user_id))
            .all(self.db.rdb())
            .await?;

        let total = results.len() as u64;
        Ok((results, total))
    }
}
