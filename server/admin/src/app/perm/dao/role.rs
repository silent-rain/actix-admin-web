//! 角色管理
use crate::app::perm::dto::role::RoleListReq;

use database::{DbRepo, Pagination};
use entity::{perm_role, prelude::PermRole};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

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
    pub async fn list(&self, req: RoleListReq) -> Result<(Vec<perm_role::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermRole::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_role::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_role::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;

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

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_role::ActiveModel,
    ) -> Result<perm_role::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, data: perm_role::Model) -> Result<u64, DbErr> {
        // Into ActiveModel
        let pear: perm_role::ActiveModel = data.clone().into();

        let result = PermRole::update_many()
            .set(pear)
            .filter(perm_role::Column::Id.eq(data.id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermRole::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
