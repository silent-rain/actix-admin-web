//! 作业
use std::time::Duration;

use chrono::Local;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};
use uuid::Uuid;

pub struct XJob {
    job: Job,
}

impl XJob {
    /// 添加定时任务作业
    pub fn cron_job(schedule: &str) -> Result<Self, JobSchedulerError> {
        let job = Job::new_async_tz(schedule, Local, |uuid, mut _jl| {
            Box::pin(async move {
                println!("I run async every 7 seconds uuid: {uuid}");
            })
        })?;

        Ok(XJob { job })
    }

    /// 添加即时任务作业
    pub fn interval_job(secs: u64) -> Result<Self, JobSchedulerError> {
        let job = Job::new_repeated_async(Duration::from_secs(secs), move |uuid, _jl| {
            let secs1 = secs;

            Box::pin(async move {
                println!(
                    "{:?} I'm repeated every {secs1} seconds uuid: {uuid}",
                    chrono::Local::now()
                );
            })
        })?;

        Ok(XJob { job })
    }

    pub fn form_job(job: Job) -> Self {
        XJob { job }
    }

    /// 重置已有定时任务
    pub fn form_cron_uuid(uuid: &str, schedule: &str) -> Result<Self, JobSchedulerError> {
        let job_id = Uuid::parse_str(uuid).map_err(|_err| JobSchedulerError::ErrorLoadingJob)?;
        let job = JobBuilder::new()
            .with_timezone(Local)
            .with_cron_job_type()
            .with_job_id(job_id.into())
            .with_schedule(schedule)?
            .with_run_async(Box::new(|uuid, mut _l| {
                Box::pin(async move {
                    println!("form_cron_uuid JHB run async every 2 seconds id {:?}", uuid);
                })
            }))
            .build()?;
        Ok(XJob { job })
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
        self.job
            .on_start_notification_add(
                &sched,
                Box::new(|job_id, notification_id, type_of_notification| {
                    Box::pin(async move {
                        println!(
                            "Job {:?} was started, notification {:?} ran ({:?})",
                            job_id, notification_id, type_of_notification
                        );
                    })
                }),
            )
            .await?;
        Ok(())
    }

    // 添加作业停止时要执行的操作
    pub async fn on_stop_notification(
        &mut self,
        sched: JobScheduler,
    ) -> Result<(), JobSchedulerError> {
        self.job
            .on_stop_notification_add(
                &sched,
                Box::new(|job_id, notification_id, type_of_notification| {
                    Box::pin(async move {
                        println!(
                            "Job {:?} was stop, notification {:?} ran ({:?})",
                            job_id, notification_id, type_of_notification
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
        self.job
            .on_done_notification_add(
                &sched,
                Box::new(|job_id, notification_id, type_of_notification| {
                    Box::pin(async move {
                        println!(
                            "Job {:?} was done, notification {:?} ran ({:?})",
                            job_id, notification_id, type_of_notification
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
        self.job
            .on_removed_notification_add(
                &sched,
                Box::new(|job_id, notification_id, type_of_notification| {
                    Box::pin(async move {
                        println!(
                            "Job {:?} was removed, notification {:?} ran ({:?})",
                            job_id, notification_id, type_of_notification
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
        self.on_stop_notification(sched.clone()).await?;
        self.on_done_notification(sched.clone()).await?;
        self.on_removed_notification(sched.clone()).await?;
        Ok(())
    }
}
