//! 角色管理
use crate::app::perm::dto::role::{AddRoleReq, RoleListReq};

use database::{DbRepo, Pagination};
use entity::{
    perm_role, perm_user_role_rel,
    prelude::{PermRole, PermUserRoleRel},
};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
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

        let paginator = PermRole::find()
            .order_by_asc(perm_role::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let num_pages = paginator.num_items().await?;

        let results = paginator.fetch_page(page.page()).await?;

        Ok((results, num_pages))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_role::Model>, DbErr> {
        PermRole::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(&self, data: AddRoleReq) -> Result<perm_role::Model, DbErr> {
        let pear = perm_role::ActiveModel {
            name: Set(data.name),
            note: Set(data.note),
            status: Set(1_i8),
            sort: Set(1_i32),
            ..Default::default() // all other attributes are `NotSet`
        };

        pear.insert(self.db.wdb()).await
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

impl<'a> RoleDao<'a> {
    /// 通过用户ID获取角色列表
    pub async fn role_list(&self, user_id: i32) -> Result<(Vec<perm_role::Model>, u64), DbErr> {
        let results = PermRole::find()
            .left_join(PermUserRoleRel)
            .filter(perm_user_role_rel::Column::UserId.eq(user_id))
            .order_by_asc(perm_role::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;

        Ok((results, total))
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, QuerySelect, QueryTrait};

    use super::*;

    #[test]
    fn test_role_list() {
        let result = PermRole::find()
            .select_only()
            .columns([perm_role::Column::Id])
            .left_join(PermUserRoleRel)
            .filter(perm_user_role_rel::Column::UserId.eq(10))
            .order_by_asc(perm_role::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "perm_role"."id" FROM "perm_role" LEFT JOIN "perm_user_role_rel" ON "perm_role"."id" = "perm_user_role_rel"."role_id" WHERE "perm_user_role_rel"."user_id" = 10 ORDER BY "perm_role"."id" ASC"#;

        assert_eq!(result, sql);
    }
}
