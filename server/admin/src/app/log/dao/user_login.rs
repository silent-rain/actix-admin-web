//! 登陆日志

use crate::app::log::dto::user_login::GetUserLoginListReq;

use database::{DbRepo, Pagination};
use entity::log_user_login;
use entity::prelude::LogUserLogin;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

#[injectable]
pub struct UserLoginDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserLoginDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserLoginListReq,
    ) -> Result<(Vec<log_user_login::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = LogUserLogin::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(log_user_login::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(log_user_login::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;

        let results = states
            .order_by_desc(log_user_login::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<log_user_login::Model>, DbErr> {
        LogUserLogin::find_by_id(id).one(self.db.rdb()).await
    }

    /// 根据用户ID获取详情信息
    pub async fn info_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<log_user_login::Model>, DbErr> {
        LogUserLogin::find()
            .filter(log_user_login::Column::UserId.eq(user_id))
            .order_by_desc(log_user_login::Column::Id)
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: log_user_login::ActiveModel,
    ) -> Result<log_user_login::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = log_user_login::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }
}
