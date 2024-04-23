//! 系统内置定时任务

pub mod demo;

use database::DbRepo;
use scheduler::{Job, JobSchedulerError};

use async_trait::async_trait;

/// 定时任务
#[async_trait]
pub trait Task<DB>
where
    DB: DbRepo + Send + Sync + 'static,
{
    /// 系统定时任务编码
    fn sys_code() -> String;
    /// 运行任务
    fn run(&mut self) -> Result<Job<DB>, JobSchedulerError>;
}
