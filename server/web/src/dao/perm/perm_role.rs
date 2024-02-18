//! 角色管理
use crate::dto::pagination::Pagination;
use crate::dto::perm::perm_role::{AddRoleReq, GetRoleListReq};

use database::DBRepo;
use entity::perm_role;
use entity::prelude::PermRole;

use nject::injectable;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, QueryFilter, Set};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

#[injectable]
pub struct PermRoleDao<'a> {
    db: &'a dyn DBRepo,
}

impl<'a> PermRoleDao<'a> {
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
