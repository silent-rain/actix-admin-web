//! 作业
//! ```rust,ignore
//! |_uuid: Uuid, _jobs: JobScheduler| -> Pin<Box<dyn Future<Output = ()> + Send>> + 'static {
//!     Box::pin(async move {})
//! }
//! ```
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{
        atomic::{
            AtomicI64,
            Ordering::{self, SeqCst},
        },
        Arc,
    },
    time::Duration,
};

use crate::{dao::Dao, enums::ScheduleJobLogStatus};

use database::DbRepo;
use entity::schedule_job_log;

use chrono::Local;
use sea_orm::Set;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};
use tracing::{error, trace};
use uuid::Uuid;

#[derive(Clone)]
pub struct XJob<JobRun, DB>
where
    JobRun: FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync
        + 'static,
    DB: DbRepo + Send + Sync + 'static,
{
    id: i32,
    job: Job,
    dao: Arc<Dao<DB>>,
    start_time: Arc<AtomicI64>,
    job_run: PhantomData<JobRun>,
}

impl<JobRun, DB> XJob<JobRun, DB>
where
    JobRun: FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync
        + 'static,
    DB: DbRepo + Send + Sync + 'static,
{
    pub fn new(id: i32, db: DB) -> Result<Self, JobSchedulerError> {
        let job = XJob {
            id,
            job: Job::new_one_shot(Duration::from_secs(0), |_uuid, _jobs| {})?,
            dao: Arc::new(Dao::new(db)),
            start_time: Arc::new(AtomicI64::new(0)),
            job_run: std::marker::PhantomData,
        };

        Ok(job)
    }

    /// 添加定时任务作业
    pub fn with_cron_job(mut self, schedule: &str, run: JobRun) -> Result<Self, JobSchedulerError> {
        let job = Job::new_async_tz(schedule, Local, run)?;

        self.job = job;

        Ok(self)
    }

    /// 添加即时任务作业
    pub fn witch_interval_job(mut self, secs: u64, run: JobRun) -> Result<Self, JobSchedulerError> {
        let job = Job::new_repeated_async(Duration::from_secs(secs), run)?;

        self.job = job;

        Ok(self)
    }

    /// 重置已有定时任务
    pub fn witch_cron_uuid(
        mut self,
        uuid: &str,
        schedule: &str,
        run: JobRun,
    ) -> Result<Self, JobSchedulerError> {
        let job_id = Uuid::parse_str(uuid).map_err(|_err| JobSchedulerError::ErrorLoadingJob)?;
        let job = JobBuilder::new()
            .with_timezone(Local)
            .with_cron_job_type()
            .with_job_id(job_id.into())
            .with_schedule(schedule)?
            .with_run_async(Box::new(run))
            .build()?;
        self.job = job;

        Ok(self)
    }

    /// 添加指定定时任务
    pub fn form_job(mut self, job: Job) -> Self {
        self.job = job;
        self
    }

    /// 返回 UUID
    pub fn guid(&self) -> Uuid {
        self.job.guid()
    }

    /// 返回任务
    pub fn job(&self) -> Job {
        self.job.clone()
    }

    // 添加作业启动时要执行的操作
    pub async fn on_start_notification(
        &mut self,
        sched: JobScheduler,
    ) -> Result<(), JobSchedulerError> {
        let dao = self.dao.clone();
        let start_time = self.start_time.clone();
        let id = self.id;
        self.job
            .on_start_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();
                    let start_time = start_time.clone();
                    Box::pin(async move {
                        let dao = dao.clone();
                        let start_time = start_time.clone();
                        start_time.fetch_add(Local::now().timestamp_millis(), SeqCst);

                        let model = schedule_job_log::ActiveModel {
                            job_id: Set(id),
                            cost: Set(0),
                            status: Set(ScheduleJobLogStatus::Running as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("Job {:?} was started, err: {:?}", job_id, err);
                            return;
                        };

                        trace!(
                            "Job {:?} was started, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await?;
        Ok(())
    }

    // 添加作业完成时要执行的操作
    pub async fn on_done_notification(
        &mut self,
        sched: JobScheduler,
    ) -> Result<(), JobSchedulerError> {
        let dao = self.dao.clone();
        let start_time = self.start_time.clone();
        let id = self.id;

        self.job
            .on_done_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();
                    let start_time = start_time.clone();

                    Box::pin(async move {
                        let dao = dao.clone();
                        let start_time = start_time.clone();
                        let cost =
                            Local::now().timestamp_millis() - start_time.load(Ordering::Relaxed);

                        let model = schedule_job_log::ActiveModel {
                            job_id: Set(id),
                            cost: Set(cost),
                            status: Set(ScheduleJobLogStatus::Success as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("Job {:?} was success, err: {:?}", job_id, err);
                            return;
                        };
                        trace!(
                            "Job {:?} was success, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await?;
        Ok(())
    }

    // 添加作业移除时要执行的操作
    pub async fn on_removed_notification(
        &mut self,
        sched: JobScheduler,
    ) -> Result<(), JobSchedulerError> {
        let dao = self.dao.clone();
        let start_time = self.start_time.clone();
        let id = self.id;

        self.job
            .on_removed_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();
                    let start_time = start_time.clone();

                    Box::pin(async move {
                        let dao = dao.clone();
                        let start_time = start_time.clone();
                        let cost =
                            Local::now().timestamp_millis() - start_time.load(Ordering::Relaxed);

                        let model = schedule_job_log::ActiveModel {
                            job_id: Set(id),
                            cost: Set(cost),
                            status: Set(ScheduleJobLogStatus::Removed as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("Job {:?} was removed, err: {:?}", job_id, err);
                            return;
                        };

                        trace!(
                            "Job {:?} was removed, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await?;
        Ok(())
    }

    /// 设置任务消息通知事件
    pub async fn set_job_notification(
        &mut self,
        sched: JobScheduler,
    ) -> Result<(), JobSchedulerError> {
        self.on_start_notification(sched.clone()).await?;
        self.on_done_notification(sched.clone()).await?;
        self.on_removed_notification(sched.clone()).await?;
        Ok(())
    }
}
