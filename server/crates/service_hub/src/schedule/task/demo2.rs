//! 定时任务示例2
use database::DbRepo;
use scheduler::{Job, JobSchedulerError};

use super::TaskTrait;

pub struct DemoTask2<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    db: DB,
}

impl<DB> TaskTrait<DB> for DemoTask2<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
    Self: Sized,
{
    fn new(db: DB) -> Box<dyn TaskTrait<DB>> {
        Box::new(DemoTask2 { db })
    }

    fn sys_code(&self) -> String {
        "task_demo2".to_owned()
    }

    fn task(&self) -> Result<Job<DB>, JobSchedulerError> {
        let job = Job::new(1, self.db.clone())?.witch_interval_job(8, |uuid, _jobs| {
            Box::pin(async move {
                println!("I run async demo uuid: {uuid}");
            })
        })?;

        Ok(job)
    }
}
