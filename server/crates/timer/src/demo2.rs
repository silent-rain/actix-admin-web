//! 定时任务示例2
use database::DbRepo;
use entity::schedule_job;
use scheduler::{enums::ScheduleJobSource, error::Error, register::SysTaskTrait, Job};

pub struct DemoTask2<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    db: DB,
}

impl<DB> DemoTask2<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Sized,
{
    pub fn new(db: DB) -> Self {
        DemoTask2 { db }
    }
}

impl<DB> SysTaskTrait<DB> for DemoTask2<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Sized,
{
    fn sys_code(&self) -> String {
        "task_demo2".to_owned()
    }

    fn task(&self, job_model: schedule_job::Model) -> Result<Job<DB>, Error> {
        if job_model.source != ScheduleJobSource::System as i8 {
            return Err(Error::ModelSourceError);
        }
        let expression = job_model.expression.ok_or(Error::NotExpressionError)?;

        let job = Job::new(1, self.db.clone())?.with_cron_job(&expression, |uuid, _jobs| {
            Box::pin(async move {
                println!("I run async expression demo uuid: {uuid}");
            })
        })?;

        Ok(job)
    }
}
