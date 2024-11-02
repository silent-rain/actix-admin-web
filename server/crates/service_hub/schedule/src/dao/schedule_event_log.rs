//! 任务调度事件日志管理

use std::sync::Arc;

use crate::dto::schedule_event_log::GetScheduleEventLogListReq;

use database::{Pagination, PoolTrait};
use entity::schedule::{schedule_event_log, ScheduleEventLog};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct ScheduleEventLogDao {
    db: Arc<dyn PoolTrait>,
}

impl ScheduleEventLogDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetScheduleEventLogListReq,
    ) -> Result<(Vec<schedule_event_log::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ScheduleEventLog::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(schedule_event_log::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(schedule_event_log::Column::CreatedAt.lt(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_event_log::Column::JobId.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(schedule_event_log::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<schedule_event_log::Model>, DbErr> {
        ScheduleEventLog::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: schedule_event_log::ActiveModel,
    ) -> Result<schedule_event_log::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ScheduleEventLog::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
