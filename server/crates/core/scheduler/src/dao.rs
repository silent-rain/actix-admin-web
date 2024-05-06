//! 数据库操作
use database::DbRepo;
use entity::prelude::ScheduleJob;
use entity::schedule_job;
use entity::schedule_job_event_log;

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

pub struct Dao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    db: DB,
}

impl<DB: DbRepo> Dao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    /// 创建对象
    pub fn new(db: DB) -> Self {
        Dao { db }
    }

    /// 获取调度任务列表
    pub async fn get_schedule_job_list(&self) -> Result<Vec<schedule_job::Model>, DbErr> {
        ScheduleJob::find().all(self.db.rdb()).await
    }

    /// 添加调度任务状态日志
    pub async fn add_schedule_job_status_log(
        &self,
        active_model: schedule_job::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleJob::update_many()
            .set(active_model)
            .filter(schedule_job::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 更新调度任务状态日志
    pub async fn update_schedule_job_status_log(
        &self,
        active_model: schedule_job::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleJob::update_many()
            .set(active_model)
            .filter(schedule_job::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 更新任务
    pub async fn update_schedule_job(
        &self,
        active_model: schedule_job::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleJob::update_many()
            .set(active_model)
            .filter(schedule_job::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 添加任务日志
    pub async fn add_schedule_job_log(
        &self,
        active_model: schedule_job_event_log::ActiveModel,
    ) -> Result<schedule_job_event_log::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }
}
