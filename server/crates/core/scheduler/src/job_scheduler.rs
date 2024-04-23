//! 定时任务调度
use std::sync::OnceLock;

use crate::job::XJob;

use database::DbRepo;

use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};
use tracing::info;
use uuid::Uuid;

/// 全局调度对象
static GLOBAL_SCHED: OnceLock<JobScheduler> = OnceLock::new();

pub struct XJobScheduler {
    sched: JobScheduler,
}

impl XJobScheduler {
    /// 初始化任务调度对象
    pub async fn new() -> Result<Self, JobSchedulerError> {
        let sched = JobScheduler::new().await?;
        GLOBAL_SCHED.get_or_init(|| sched.clone());
        Ok(XJobScheduler { sched })
    }

    pub fn from(sched: JobScheduler) -> Self {
        XJobScheduler { sched }
    }

    /// 获取任务调度对象
    pub fn instance() -> Result<JobScheduler, JobSchedulerError> {
        GLOBAL_SCHED
            .get()
            .cloned()
            .ok_or(JobSchedulerError::ParseSchedule)
    }

    /// 将job添加到定时器中
    pub async fn add_job<DB>(&self, mut xjob: XJob<DB>) -> Result<Uuid, JobSchedulerError>
    where
        DB: DbRepo + Send + Sync + 'static,
    {
        xjob.set_job_notification(self.sched.clone()).await?;
        self.sched.add(xjob.job()).await
    }

    /// 添加要在关闭期间/之后运行的代码
    pub fn set_shutdown_handler(&mut self) {
        self.sched.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                info!("job scheduler shutdown done");
            })
        }));
    }

    /// 移除Job任务
    pub async fn remove(&self, job_id: &Uuid) -> Result<(), JobSchedulerError> {
        self.sched.remove(job_id).await?;
        info!("remove job...");
        Ok(())
    }

    /// 启动调度程序
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        self.sched.start().await?;
        info!("job scheduler start...");
        Ok(())
    }

    /// 关闭调度程序
    pub async fn shutdown(&mut self) -> Result<(), JobSchedulerError> {
        self.sched.shutdown().await?;
        info!("job scheduler shutdown...");
        Ok(())
    }
}
