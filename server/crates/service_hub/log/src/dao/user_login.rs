//! 登陆日志管理

use crate::dto::user_login::GetUserLoginListReq;

use database::{ArcDbRepo, Pagination};
use entity::log_user_login;
use entity::prelude::LogUserLogin;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct UserLoginDao {
    db: ArcDbRepo,
}

impl UserLoginDao {
    pub fn new(db: ArcDbRepo) -> Self {
        UserLoginDao { db }
    }

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
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(log_user_login::Column::UserId.eq(v))
            })
            .apply_if(req.username, |query, v| {
                query.filter(log_user_login::Column::Username.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

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

    /// 根据Token获取详情信息
    pub async fn info_by_token(
        &self,
        token: String,
    ) -> Result<Option<log_user_login::Model>, DbErr> {
        LogUserLogin::find()
            .filter(log_user_login::Column::Token.eq(token))
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

    /// 更新数据
    pub async fn update(&self, active_model: log_user_login::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = LogUserLogin::update_many()
            .set(active_model)
            .filter(log_user_login::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新禁用状态
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
