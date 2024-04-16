//! 角色管理
use crate::perm::dto::role::GetRoleListReq;

use database::{DbRepo, Pagination};
use entity::{perm_role, prelude::PermRole};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct RoleDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> RoleDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<perm_role::Model>, u64), DbErr> {
        let results = PermRole::find()
            .order_by_asc(perm_role::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(&self, req: GetRoleListReq) -> Result<(Vec<perm_role::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermRole::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_role::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_role::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(perm_role::Column::Name.like(format!("%{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_role::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_role::Model>, DbErr> {
        PermRole::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(&self, name: String) -> Result<Option<perm_role::Model>, DbErr> {
        PermRole::find()
            .filter(perm_role::Column::Name.eq(name))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_role::ActiveModel,
    ) -> Result<perm_role::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: perm_role::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermRole::update_many()
            .set(active_model)
            .filter(perm_role::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = perm_role::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermRole::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
