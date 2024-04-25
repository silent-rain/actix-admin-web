//! 定时任务示例2
use database::DbRepo;
use scheduler::{register::SysTaskTrait, Job, JobSchedulerError};

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

    fn task(&self) -> Result<Job<DB>, JobSchedulerError> {
        let job = Job::new(1, self.db.clone())?.with_interval_job(8, |uuid, _jobs| {
            Box::pin(async move {
                println!("I run async demo uuid: {uuid}");
            })
        })?;

        Ok(job)
    }
}
