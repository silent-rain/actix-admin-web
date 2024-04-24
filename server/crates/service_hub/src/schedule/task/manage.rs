//! # 任务管理与注册
//! - 初始化所有系统任务
//!     - 获取系统任务
//!     - 通过编码更新任务的UUID
//! 初始化所有的脚本任务
use super::error::Error;
use super::{demo::DemoTask, demo2::DemoTask2, TaskTrait};
use crate::schedule::enums::ScheduleJobSource;

use database::DbRepo;
use entity::schedule_job;
use scheduler::dao::Dao;
use sea_orm::Set;

pub struct TaskManage<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    tasks: Vec<Box<dyn TaskTrait<DB>>>,
    db: DB,
    dao: Dao<DB>,
}

impl<DB> TaskManage<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        TaskManage {
            db: db.clone(),
            dao: Dao::new(db),
            tasks: Vec::new(),
        }
    }

    /// 统一注册任务的位置
    pub fn register(&mut self) {
        self.add_task(DemoTask::new(self.db.clone()));
        self.add_task(DemoTask2::new(self.db.clone()));
    }

    /// 添加任务
    fn add_task(&mut self, task: Box<dyn TaskTrait<DB>>) {
        self.tasks.push(task);
    }

    /// 开始运行任务
    pub async fn run(&mut self) -> Result<(), Error> {
        let sys_job_list = self.sys_job_list().await?;

        for job in self.tasks.iter_mut() {
            let job_model = sys_job_list
                .iter()
                .find(|v| v.sys_code != Some(job.sys_code()));
            let job_task = job
                .task()
                .map_err(|err| Error::JobSchedulerError(err.to_string()))?;
            let uuid = job_task.guid().to_string();
            let job_model = match job_model {
                Some(v) => v,
                None => continue,
            };
            // 更新为当前任务的UUID
            // self.update_schedule_job_uuid(job_model.clone(), uuid)
            //     .await?;
        }

        Ok(())
    }

    /// 获取所有的系统定时任务
    async fn sys_job_list(&self) -> Result<Vec<schedule_job::Model>, Error> {
        let job_list = self
            .dao
            .get_schedule_job_list()
            .await
            .map_err(|err| Error::ScheduleJobListError(err.to_string()))?;

        let sys_job_list = job_list
            .into_iter()
            .filter(|v| v.source == ScheduleJobSource::System as i8)
            .collect::<Vec<schedule_job::Model>>();

        Ok(sys_job_list)
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
