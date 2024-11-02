//! API操作日志

use std::sync::Arc;

use crate::dto::api_operation::GetApiOperationListReq;

use database::{Pagination, PoolTrait};
use entity::log_api_operation;
use entity::prelude::LogApiOperation;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct ApiOperationDao {
    db: Arc<dyn PoolTrait>,
}

impl ApiOperationDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetApiOperationListReq,
    ) -> Result<(Vec<log_api_operation::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = LogApiOperation::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(log_api_operation::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(log_api_operation::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(log_api_operation::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<log_api_operation::Model>, DbErr> {
        LogApiOperation::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: log_api_operation::ActiveModel,
    ) -> Result<log_api_operation::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = LogApiOperation::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
