//! 系统日志

use std::sync::Arc;

use crate::dto::system::GetSystemListReq;

use database::{Pagination, PoolTrait};
use entity::log_system;
use entity::prelude::LogSystem;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct SystemDao {
    db: Arc<dyn PoolTrait>,
}

impl SystemDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetSystemListReq,
    ) -> Result<(Vec<log_system::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = LogSystem::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(log_system::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(log_system::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(log_system::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<log_system::Model>, DbErr> {
        LogSystem::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn add(&self, data: log_system::Model) -> Result<log_system::Model, DbErr> {
        let mut active_model: log_system::ActiveModel = data.into();
        active_model.id = NotSet;
        active_model.insert(self.db.db()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = LogSystem::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
