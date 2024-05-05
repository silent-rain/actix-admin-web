//! 定时任务示例
use database::DbRepo;
use entity::schedule_job;
use scheduler::{enums::ScheduleJobSource, error::Error, register::SysTaskTrait, Job};

pub struct DemoTask<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    db: DB,
}

impl<DB> DemoTask<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Sized,
{
    pub fn new(db: DB) -> Self {
        DemoTask { db }
    }
}

impl<DB> SysTaskTrait<DB> for DemoTask<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Sized,
{
    fn sys_code(&self) -> String {
        "task_demo".to_owned()
    }

    fn task(&self, job_model: schedule_job::Model) -> Result<Job<DB>, Error> {
        if job_model.source != ScheduleJobSource::System as i8 {
            return Err(Error::ModelSourceError);
        }
        let interval = job_model.interval.ok_or(Error::NotIntervalError)?;

        let job =
            Job::new(1, self.db.clone())?.with_interval_job(interval as u64, |uuid, _jobs| {
                Box::pin(async move {
                    println!("I run async interval demo uuid: {uuid}");
                })
            })?;

        Ok(job)
    }
}
