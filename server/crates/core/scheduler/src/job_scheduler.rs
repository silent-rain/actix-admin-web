//! 定时任务调度
use std::sync::OnceLock;

use crate::{error::Error, job::Job};

use database::DbRepo;

use tokio_cron_scheduler::JobScheduler as TokioJobScheduler;
use tracing::info;
use uuid::Uuid;

/// 全局调度对象
static GLOBAL_SCHED: OnceLock<TokioJobScheduler> = OnceLock::new();

pub struct JobScheduler {
    sched: TokioJobScheduler,
}

impl JobScheduler {
    /// 初始化任务调度对象
    pub async fn new() -> Result<Self, Error> {
        let sched = match GLOBAL_SCHED.get() {
            Some(v) => v,
            None => {
                let sched = TokioJobScheduler::new()
                    .await
                    .map_err(|err| Error::InitScheduleInstance(err.to_string()))?;
                let sched = GLOBAL_SCHED.get_or_init(|| sched.clone());
                sched
            }
        };

        Ok(JobScheduler {
            sched: sched.clone(),
        })
    }

    pub fn from(sched: TokioJobScheduler) -> Self {
        JobScheduler { sched }
    }

    /// 将job添加到定时器中
    pub async fn add_job<DB>(&self, mut job: Job<DB>) -> Result<Uuid, Error>
    where
        DB: DbRepo + Send + Sync + 'static,
    {
        // 设置任务通知
        // job.set_job_notification(self.sched.clone()).await?;

        self.sched
            .add(job.job())
            .await
            .map_err(Error::JobSchedulerError)
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
    pub async fn remove(&self, job_id: &Uuid) -> Result<(), Error> {
        self.sched
            .remove(job_id)
            .await
            .map_err(Error::JobSchedulerError)?;
        info!("remove job...");
        Ok(())
    }

    /// 启动调度程序
    pub async fn start(&self) -> Result<(), Error> {
        self.sched.start().await.map_err(Error::JobSchedulerError)?;
        info!("job scheduler start...");
        Ok(())
    }

    /// 关闭调度程序
    pub async fn shutdown(&mut self) -> Result<(), Error> {
        self.sched
            .shutdown()
            .await
            .map_err(Error::JobSchedulerError)?;
        info!("job scheduler shutdown...");
        Ok(())
    }
}
