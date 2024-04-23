//! 定时任务示例
use database::DbRepo;
use scheduler::{Job, JobSchedulerError};

use super::Task;

pub struct DemoTask<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    db: DB,
}

impl<DB> Task<DB> for DemoTask<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    fn sys_code() -> String {
        todo!()
    }

    fn run(&mut self) -> Result<Job<DB>, JobSchedulerError> {
        let job = Job::new(1, self.db.clone())?.witch_interval_job(8, |uuid, _jobs| {
            Box::pin(async move {
                println!("I run async demo uuid: {uuid}");
            })
        })?;

        Ok(job)
    }
}
