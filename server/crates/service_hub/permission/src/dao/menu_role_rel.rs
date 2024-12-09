//! 菜单角色关系管理

use std::sync::Arc;

use crate::dto::menu_role_rel::GetMenuRoleRelListReq;

use database::{Pagination, PoolTrait};
use entity::{permission::menu_role_rel, permission::MenuRoleRel};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct MenuRoleRelDao {
    db: Arc<dyn PoolTrait>,
}

impl MenuRoleRelDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetMenuRoleRelListReq,
    ) -> Result<(Vec<menu_role_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = MenuRoleRel::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(menu_role_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(menu_role_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.menu_id, |query, v| {
                query.filter(menu_role_rel::Column::MenuId.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(menu_role_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: menu_role_rel::ActiveModel,
    ) -> Result<menu_role_rel::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<menu_role_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = MenuRoleRel::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = MenuRoleRel::delete_many()
            .filter(menu_role_rel::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = MenuRoleRel::delete_many()
            .filter(menu_role_rel::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
