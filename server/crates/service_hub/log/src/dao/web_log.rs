//! WEB日志管理

use crate::dto::web_log::GetWebLogListReq;

use database::{ArcDbRepo, Pagination};
use entity::log_web;
use entity::prelude::LogWeb;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct WebLogDao {
    db: ArcDbRepo,
}

impl WebLogDao {
    /// 获取数据列表
    pub async fn list(&self, req: GetWebLogListReq) -> Result<(Vec<log_web::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = LogWeb::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(log_web::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(log_web::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(log_web::Column::UserId.eq(v))
            })
            .apply_if(req.username, |query, v| {
                query.filter(log_web::Column::Username.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(log_web::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<log_web::Model>, DbErr> {
        LogWeb::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: log_web::ActiveModel) -> Result<log_web::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: log_web::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = LogWeb::update_many()
            .set(active_model)
            .filter(log_web::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }
}
