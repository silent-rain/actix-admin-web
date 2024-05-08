//! 数据库操作
use database::DbRepo;
use entity::{
    prelude::{ScheduleJob, ScheduleJobStatusLog},
    schedule_job, schedule_job_event_log, schedule_job_status_log,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

pub struct Dao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    pub schedule_job_dao: ScheduleJobDao<DB>,
    pub schedule_job_status_log_dao: ScheduleJobStatusLogDao<DB>,
    pub schedule_job_event_log_dao: ScheduleJobEventLogDao<DB>,
}

impl<DB: DbRepo> Dao<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    /// 创建对象
    pub fn new(db: DB) -> Self {
        Dao {
            schedule_job_dao: ScheduleJobDao::new(db.clone()),
            schedule_job_status_log_dao: ScheduleJobStatusLogDao::new(db.clone()),
            schedule_job_event_log_dao: ScheduleJobEventLogDao::new(db.clone()),
        }
    }
}

pub struct ScheduleJobDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    db: DB,
}

impl<DB: DbRepo> ScheduleJobDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    /// 创建对象
    pub fn new(db: DB) -> Self {
        ScheduleJobDao { db }
    }

    /// 获取任务调度列表
    pub async fn list(&self) -> Result<Vec<schedule_job::Model>, DbErr> {
        ScheduleJob::find().all(self.db.rdb()).await
    }
}

pub struct ScheduleJobStatusLogDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    db: DB,
}

impl<DB: DbRepo> ScheduleJobStatusLogDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    /// 创建对象
    pub fn new(db: DB) -> Self {
        ScheduleJobStatusLogDao { db }
    }

    /// 添加任务调度状态日志
    pub async fn add(
        &self,
        job_id: i32,
        uuid: String,
    ) -> Result<schedule_job_status_log::Model, DbErr> {
        let active_model = schedule_job_status_log::ActiveModel {
            job_id: Set(job_id),
            uuid: Set(uuid),
            cost: Set(0),
            status: Set(schedule_job_status_log::enums::Status::Start as i8),
            ..Default::default()
        };
        active_model.insert(self.db.wdb()).await
    }

    /// 更新任务调度状态日志
    pub async fn update(
        &self,
        id: i32,
        cost: u64,
        error: Option<String>,
        status: schedule_job_status_log::enums::Status,
    ) -> Result<u64, DbErr> {
        let active_model = schedule_job_status_log::ActiveModel {
            id: Set(id),
            error: Set(error),
            cost: Set(cost),
            status: Set(status as i8),
            ..Default::default()
        };
        let result = ScheduleJobStatusLog::update_many()
            .set(active_model)
            .filter(schedule_job_status_log::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }

    /// 更新任务调度状态日志状态
    pub async fn status(
        &self,
        id: i32,
        status: schedule_job_status_log::enums::Status,
    ) -> Result<u64, DbErr> {
        let active_model = schedule_job_status_log::ActiveModel {
            id: Set(id),
            status: Set(status as i8),
            ..Default::default()
        };
        let result = ScheduleJobStatusLog::update_many()
            .set(active_model)
            .filter(schedule_job_status_log::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}

pub struct ScheduleJobEventLogDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    db: DB,
}

impl<DB: DbRepo> ScheduleJobEventLogDao<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    /// 创建对象
    pub fn new(db: DB) -> Self {
        ScheduleJobEventLogDao { db }
    }

    /// 添加任务调度事件日志
    pub async fn add(
        &self,
        job_id: i32,
        uuid: String,
        status: schedule_job_event_log::enums::Status,
    ) -> Result<schedule_job_event_log::Model, DbErr> {
        let active_model = schedule_job_event_log::ActiveModel {
            job_id: Set(job_id),
            uuid: Set(uuid),
            status: Set(status as i8),
            ..Default::default()
        };
        active_model.insert(self.db.wdb()).await
    }
}
