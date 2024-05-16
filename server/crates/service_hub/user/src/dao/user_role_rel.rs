//! 用户角色关系管理

use crate::dto::user_role_rel::GetUserRoleRelListReq;

use database::{DbRepo, Pagination};
use entity::user::{user_role_rel, UserRoleRel};

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
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let results = UserRoleRel::find()
            .order_by_asc(user_role_rel::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserRoleRelListReq,
    ) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserRoleRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_role_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_role_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(user_role_rel::Column::UserId.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_role_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: user_role_rel::ActiveModel,
    ) -> Result<user_role_rel::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<user_role_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = UserRoleRel::insert_many(active_models)
            .exec(self.db.wdb())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = UserRoleRel::delete_many()
            .filter(user_role_rel::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = UserRoleRel::delete_many()
            .filter(user_role_rel::Column::Id.is_in(ids))
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
    ) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let results = UserRoleRel::find()
            .filter(user_role_rel::Column::UserId.eq(user_id))
            .all(self.db.rdb())
            .await?;

        let total = results.len() as u64;
        Ok((results, total))
    }
}
