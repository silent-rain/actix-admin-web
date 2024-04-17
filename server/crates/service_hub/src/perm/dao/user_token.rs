//! 用户Token令牌管理
use crate::perm::dto::user_token::GetUserTokenListReq;

use database::{DbRepo, Pagination};
use entity::{perm_user_token, prelude::PermUserToken};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct UserTokenDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserTokenDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserTokenListReq,
    ) -> Result<(Vec<perm_user_token::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermUserToken::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_user_token::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_user_token::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(perm_user_token::Column::UserId.eq(v))
            })
            .apply_if(req.token, |query, v| {
                query.filter(perm_user_token::Column::Token.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_user_token::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user_token::Model>, DbErr> {
        PermUserToken::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_user_token::ActiveModel,
    ) -> Result<perm_user_token::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: perm_user_token::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermUserToken::update_many()
            .set(active_model)
            .filter(perm_user_token::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = perm_user_token::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUserToken::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
