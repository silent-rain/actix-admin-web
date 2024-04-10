//! 角色部门关系管理

use crate::perm::dto::role_dept_rel::GetRoleDeptRelListReq;

use database::{DbRepo, Pagination};
use entity::{perm_role_dept_rel, prelude::PermRoleDeptRel};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct RoleDeptRelDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> RoleDeptRelDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetRoleDeptRelListReq,
    ) -> Result<(Vec<perm_role_dept_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermRoleDeptRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_role_dept_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_role_dept_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.role_id, |query, v| {
                query.filter(perm_role_dept_rel::Column::RoleId.like(format!("%{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;

        let results = states
            .order_by_desc(perm_role_dept_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 添加关联关系
    pub async fn add(
        &self,
        active_model: perm_role_dept_rel::ActiveModel,
    ) -> Result<perm_role_dept_rel::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<perm_role_dept_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = PermRoleDeptRel::insert_many(active_models)
            .exec(self.db.wdb())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermRoleDeptRel::delete_many()
            .filter(perm_role_dept_rel::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = PermRoleDeptRel::delete_many()
            .filter(perm_role_dept_rel::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
