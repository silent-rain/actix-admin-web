//! 任务调度状态日志管理

use crate::dto::schedule_job_status_log::GetScheduleJobStatusListLogReq;

use database::{DbRepo, Pagination};
use entity::prelude::ScheduleJobStatusLog;
use entity::schedule_job_status_log;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct ScheduleJobStatusLogDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> ScheduleJobStatusLogDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetScheduleJobStatusListLogReq,
    ) -> Result<(Vec<schedule_job_status_log::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ScheduleJobStatusLog::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(schedule_job_status_log::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(schedule_job_status_log::Column::CreatedAt.lt(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_job_status_log::Column::JobId.eq(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_job_status_log::Column::JobId.eq(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(schedule_job_status_log::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(schedule_job_status_log::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<schedule_job_status_log::Model>, DbErr> {
        ScheduleJobStatusLog::find_by_id(id)
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: schedule_job_status_log::ActiveModel,
    ) -> Result<schedule_job_status_log::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(
        &self,
        active_model: schedule_job_status_log::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleJobStatusLog::update_many()
            .set(active_model)
            .filter(schedule_job_status_log::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = schedule_job_status_log::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ScheduleJobStatusLog::delete_by_id(id)
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
