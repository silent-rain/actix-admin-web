//! 作业
//! ```rust,ignore
//! |_uuid: Uuid, _jobs: JobScheduler| -> Pin<Box<dyn Future<Output = ()> + Send>> + 'static {
//!     Box::pin(async move {})
//! }
//! ```
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use crate::{dao::Dao, enums::ScheduleJobLogStatus, error::Error};

use database::DbRepo;
use entity::schedule_job_event_log;

use chrono::Local;
use sea_orm::Set;
use tokio_cron_scheduler::{Job as TokioJob, JobBuilder, JobScheduler, JobSchedulerError};
use tracing::{error, trace};
use uuid::Uuid;

#[derive(Clone)]
pub struct Job<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    dao: Arc<Dao<DB>>,
    id: i32,
    job: TokioJob,
}

impl<DB> Job<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    pub fn new(id: i32, db: DB) -> Result<Self, Error> {
        let job = Job {
            id,
            job: TokioJob::new_one_shot(Duration::from_secs(0), |_uuid, _jobs| {})
                .map_err(Error::JobSchedulerError)?,
            dao: Arc::new(Dao::new(db)),
        };

        Ok(job)
    }

    /// 添加定时任务作业
    pub fn with_cron_job<JobRun>(mut self, schedule: &str, run: JobRun) -> Result<Self, Error>
    where
        JobRun: FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let job = TokioJob::new_async_tz(schedule, Local, run).map_err(Error::JobSchedulerError)?;
        self.job = job;

        Ok(self)
    }

    /// 添加即时任务作业
    pub fn with_interval_job<JobRun>(mut self, secs: u64, run: JobRun) -> Result<Self, Error>
    where
        JobRun: FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let job = TokioJob::new_repeated_async(Duration::from_secs(secs), run)
            .map_err(Error::JobSchedulerError)?;
        self.job = job;

        Ok(self)
    }

    /// 重置已有定时任务
    pub fn with_cron_uuid<JobRun>(
        mut self,
        uuid: &str,
        schedule: &str,
        run: JobRun,
    ) -> Result<Self, Error>
    where
        JobRun: FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let job_id = Uuid::parse_str(uuid).map_err(|_err| JobSchedulerError::ErrorLoadingJob)?;
        let job = JobBuilder::new()
            .with_timezone(Local)
            .with_cron_job_type()
            .with_job_id(job_id.into())
            .with_schedule(schedule)?
            .with_run_async(Box::new(run))
            .build()
            .map_err(Error::JobSchedulerError)?;
        self.job = job;

        Ok(self)
    }

    /// 添加指定定时任务
    pub fn form_job(mut self, job: TokioJob) -> Self {
        self.job = job;
        self
    }

    /// 返回 UUID
    pub fn guid(&self) -> Uuid {
        self.job.guid()
    }

    /// 返回任务
    pub fn job(&self) -> TokioJob {
        self.job.clone()
    }

    // 添加作业启动时要执行的操作
    pub async fn on_start_notification(&mut self, sched: JobScheduler) -> Result<(), Error> {
        let dao = self.dao.clone();
        let id = self.id;
        self.job
            .on_start_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();
                    Box::pin(async move {
                        let model = schedule_job_event_log::ActiveModel {
                            job_id: Set(id),
                            status: Set(ScheduleJobLogStatus::Start as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("TokioJob {:?} was started, err: {:?}", job_id, err);
                            return;
                        };

                        trace!(
                            "TokioJob {:?} was started, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await
            .map_err(Error::JobSchedulerError)?;
        Ok(())
    }

    // 添加作业完成时要执行的操作
    pub async fn on_done_notification(&mut self, sched: JobScheduler) -> Result<(), Error> {
        let dao = self.dao.clone();
        let id = self.id;

        self.job
            .on_done_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();

                    Box::pin(async move {
                        let model = schedule_job_event_log::ActiveModel {
                            job_id: Set(id),
                            status: Set(ScheduleJobLogStatus::Done as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("TokioJob {:?} was done, err: {:?}", job_id, err);
                            return;
                        };
                        trace!(
                            "TokioJob {:?} was done, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await
            .map_err(Error::JobSchedulerError)?;
        Ok(())
    }

    // 添加作业停止时要执行的操作
    pub async fn on_stop_notification(&mut self, sched: JobScheduler) -> Result<(), Error> {
        let dao = self.dao.clone();
        let id = self.id;

        self.job
            .on_stop_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();

                    Box::pin(async move {
                        let model = schedule_job_event_log::ActiveModel {
                            job_id: Set(id),
                            status: Set(ScheduleJobLogStatus::Removed as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("TokioJob {:?} was stop, err: {:?}", job_id, err);
                            return;
                        };

                        trace!(
                            "TokioJob {:?} was stop, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await
            .map_err(Error::JobSchedulerError)?;
        Ok(())
    }

    // 添加作业移除时要执行的操作
    pub async fn on_removed_notification(&mut self, sched: JobScheduler) -> Result<(), Error> {
        let dao = self.dao.clone();
        let id = self.id;

        self.job
            .on_removed_notification_add(
                &sched,
                Box::new(move |job_id, notification_id, type_of_notification| {
                    let dao = dao.clone();

                    Box::pin(async move {
                        let model = schedule_job_event_log::ActiveModel {
                            job_id: Set(id),
                            status: Set(ScheduleJobLogStatus::Removed as i8),
                            ..Default::default()
                        };

                        if let Err(err) = dao.add_schedule_job_log(model).await {
                            error!("TokioJob {:?} was removed, err: {:?}", job_id, err);
                            return;
                        };

                        trace!(
                            "TokioJob {:?} was removed, notification {:?} ran ({:?})",
                            job_id,
                            notification_id,
                            type_of_notification
                        );
                    })
                }),
            )
            .await
            .map_err(Error::JobSchedulerError)?;
        Ok(())
    }

    /// 设置任务消息通知事件
    pub async fn set_job_notification(&mut self, sched: JobScheduler) -> Result<(), Error> {
        self.on_start_notification(sched.clone()).await?;
        self.on_done_notification(sched.clone()).await?;
        self.on_stop_notification(sched.clone()).await?;
        self.on_removed_notification(sched.clone()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::sync::atomic::{AtomicI64, Ordering};

    use super::*;

    #[tokio::test]
    async fn test_cost() {
        let start_time = Local::now().timestamp_millis();
        tokio::time::sleep(time::Duration::from_millis(5)).await;
        let end_time = Local::now().timestamp_millis();
        let cost = end_time - start_time;
        println!("cost: {:?}", cost as u64);
    }

    #[tokio::test]
    async fn test_atomic_i64_cost() {
        let start_time = Arc::new(AtomicI64::new(0));
        start_time.fetch_add(Local::now().timestamp_millis(), Ordering::SeqCst);
        tokio::time::sleep(time::Duration::from_millis(5)).await;
        let end_time = Local::now().timestamp_millis();
        let cost = end_time - start_time.load(Ordering::Relaxed);

        println!(
            "start_time: {:?} end_time: {}",
            start_time.load(Ordering::Relaxed),
            end_time
        );
        println!("cost: {:?}", cost as u64);
    }
}
