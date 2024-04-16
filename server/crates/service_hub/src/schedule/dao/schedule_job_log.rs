//! 定时任务

use crate::schedule::dto::schedule_job_log::GetScheduleJobLogListReq;

use database::{DbRepo, Pagination};
use entity::prelude::ScheduleJobLog;
use entity::schedule_job_log;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

/// 数据访问
#[injectable]
pub struct ScheduleJobLogDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> ScheduleJobLogDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetScheduleJobLogListReq,
    ) -> Result<(Vec<schedule_job_log::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ScheduleJobLog::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(schedule_job_log::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(schedule_job_log::Column::CreatedAt.lt(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_job_log::Column::JobId.eq(v))
            })
            .apply_if(req.job_name, |query, v| {
                query.filter(schedule_job_log::Column::JobName.like(format!("%{v}%")))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(schedule_job_log::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<schedule_job_log::Model>, DbErr> {
        ScheduleJobLog::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: schedule_job_log::ActiveModel,
    ) -> Result<schedule_job_log::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ScheduleJobLog::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
