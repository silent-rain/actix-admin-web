//! # 任务管理与注册
//! - 初始化所有系统任务
//!     - 获取系统任务
//!     - 注册系统任务
//!     - 通过编码更新任务的UUID
//! - 初始化所有的脚本任务
//!     - 注册用户任务
//!     - 通过编码更新任务的UUID
use crate::{
    dao::Dao,
    enums::{ScheduleJobSource, ScheduleJobStatus, ScheduleJobType},
    error::Error,
    Job, JobScheduler,
};

use database::DbRepo;
use entity::schedule_job;

use async_trait::async_trait;
use sea_orm::Set;
use tracing::error;

/// 系统定时任务 Trait
#[async_trait]
pub trait SysTaskTrait<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Send + Sync + 'static,
{
    /// 系统定时任务编码
    fn sys_code(&self) -> String;
    /// 执行的任务
    fn task(&self, job_model: schedule_job::Model) -> Result<Job<DB>, Error>;
}

/// 系统定时任务注册
pub struct SysTaskRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    dao: Dao<DB>,
    tasks: Vec<Box<dyn SysTaskTrait<DB>>>,
}

impl<DB> SysTaskRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        SysTaskRegister {
            dao: Dao::new(db),
            tasks: Vec::new(),
        }
    }

    /// 注册任务
    pub async fn register(&mut self) -> Result<(), Error> {
        let sys_job_list = self.sys_job_list().await?;

        for task in self.tasks.iter() {
            let job_model = sys_job_list
                .iter()
                .find(|v| v.sys_code == Some(task.sys_code()));

            // 更新数据库中的任务UUID
            let job_model = match job_model {
                Some(v) => v,
                None => continue,
            };

            let sys_job = match task.task(job_model.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
            let uuid = sys_job.guid().to_string();

            // 将任务添加到任务队列中
            let sched = match JobScheduler::new().await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
            match sched.add_job(sys_job.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };

            // 更新为当前任务的UUID
            match self.update_schedule_job_uuid(job_model.clone(), uuid).await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
        }

        Ok(())
    }

    /// 添加任务
    pub fn add_task(&mut self, task: Box<dyn SysTaskTrait<DB>>) {
        self.tasks.push(task);
    }

    /// 获取所有的系统定时任务
    async fn sys_job_list(&self) -> Result<Vec<schedule_job::Model>, Error> {
        let job_list = self
            .dao
            .get_schedule_job_list()
            .await
            .map_err(|err| Error::ScheduleJobListError(err.to_string()))?
            .into_iter()
            .filter(|v| v.status == ScheduleJobStatus::Online as i8)
            .filter(|v| v.source == ScheduleJobSource::System as i8)
            .collect::<Vec<schedule_job::Model>>();
        Ok(job_list)
    }

    // 更新为当前任务的UUID
    async fn update_schedule_job_uuid(
        &self,
        model: schedule_job::Model,
        uuid: String,
    ) -> Result<(), Error> {
        // 更新为当前任务的UUID

        let mut active_model: schedule_job::ActiveModel = model.clone().into();
        active_model.uuid = Set(Some(uuid));
        self.dao
            .update_schedule_job(active_model)
            .await
            .map_err(|err| Error::DbUpdateScheduleJobError(err.to_string()))?;
        Ok(())
    }
}

/// 用户定时任务注册
pub struct UserTaskRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    db: DB,
    dao: Dao<DB>,
}

impl<DB> UserTaskRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        UserTaskRegister {
            db: db.clone(),
            dao: Dao::new(db),
        }
    }

    /// 注册任务
    pub async fn register(&mut self) -> Result<(), Error> {
        let job_list = self.user_job_list().await?;

        for job_model in job_list.iter() {
            let user_job = if job_model.job_type == ScheduleJobType::Interval as i8 {
                self.init_interval_task(job_model)?
            } else {
                self.init_cron_task(job_model)?
            };
            let uuid = user_job.guid().to_string();

            // 将任务添加到任务队列中
            JobScheduler::new().await?.add_job(user_job.clone()).await?;
            let sched = match JobScheduler::new().await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
            match sched.add_job(user_job.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };

            // 更新为当前任务的UUID
            self.update_schedule_job_uuid(job_model.clone(), uuid)
                .await?;
        }

        Ok(())
    }

    /// 初始化定时任务
    fn init_cron_task(&self, model: &schedule_job::Model) -> Result<Job<DB>, Error> {
        let expression = model.expression.clone().ok_or(Error::NotExpressionError)?;
        let job = Job::new(1, self.db.clone())?.with_cron_job(&expression, |uuid, _jobs| {
            Box::pin(async move {
                // TODO 执行脚本
                println!("I run async every 5 seconds uuid: {uuid} job11");
            })
        })?;
        Ok(job)
    }

    /// 初始化即时任务
    fn init_interval_task(&self, model: &schedule_job::Model) -> Result<Job<DB>, Error> {
        let interval = model.interval.ok_or(Error::NotExpressionError)? as u64;
        let job = Job::new(1, self.db.clone())?.with_interval_job(interval, |uuid, _jobs| {
            Box::pin(async move {
                // TODO 执行脚本
                println!("I run async every 5 seconds uuid: {uuid} job11");
            })
        })?;

        Ok(job)
    }

    /// 获取所有的用户定时任务
    async fn user_job_list(&self) -> Result<Vec<schedule_job::Model>, Error> {
        let job_list = self
            .dao
            .get_schedule_job_list()
            .await
            .map_err(|err| Error::ScheduleJobListError(err.to_string()))?
            .into_iter()
            .filter(|v| v.status == ScheduleJobStatus::Online as i8)
            .filter(|v| v.source == ScheduleJobSource::User as i8)
            .collect::<Vec<schedule_job::Model>>();

        Ok(job_list)
    }

    // 更新为当前任务的UUID
    async fn update_schedule_job_uuid(
        &self,
        model: schedule_job::Model,
        uuid: String,
    ) -> Result<(), Error> {
        // 更新为当前任务的UUID

        let mut active_model: schedule_job::ActiveModel = model.clone().into();
        active_model.uuid = Set(Some(uuid));
        self.dao
            .update_schedule_job(active_model)
            .await
            .map_err(|err| Error::DbUpdateScheduleJobError(err.to_string()))?;
        Ok(())
    }
}
