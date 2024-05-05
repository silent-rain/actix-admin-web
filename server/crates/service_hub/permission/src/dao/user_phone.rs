//! 用户手机号管理
use crate::dto::user_phone::GetUserPhoneListReq;

use database::{DbRepo, Pagination};
use entity::{perm_user_phone, prelude::PermUserPhone};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct UserPhoneDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserPhoneDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserPhoneListReq,
    ) -> Result<(Vec<perm_user_phone::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermUserPhone::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_user_phone::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_user_phone::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(perm_user_phone::Column::UserId.eq(v))
            })
            .apply_if(req.phone, |query, v| {
                query.filter(perm_user_phone::Column::Phone.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_user_phone::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user_phone::Model>, DbErr> {
        PermUserPhone::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过手机号码获取详情信息
    pub async fn info_by_phone(
        &self,
        phone: String,
    ) -> Result<Option<perm_user_phone::Model>, DbErr> {
        PermUserPhone::find()
            .filter(perm_user_phone::Column::Phone.eq(phone))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_user_phone::ActiveModel,
    ) -> Result<perm_user_phone::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: perm_user_phone::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermUserPhone::update_many()
            .set(active_model)
            .filter(perm_user_phone::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUserPhone::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
