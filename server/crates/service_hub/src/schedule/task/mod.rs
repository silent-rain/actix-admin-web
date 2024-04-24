//! 系统内置定时任务

pub mod demo;
pub mod demo2;

pub mod error;
pub mod manage;

use database::DbRepo;
use scheduler::{Job, JobSchedulerError};

use async_trait::async_trait;

/// 定时任务
#[async_trait]
pub trait TaskTrait<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    fn new(db: DB) -> Box<dyn TaskTrait<DB>>
    where
        Self: Sized;
    /// 系统定时任务编码
    fn sys_code(&self) -> String;
    /// 执行的任务
    fn task(&self) -> Result<Job<DB>, JobSchedulerError>;
}
