//! 定时任务示例2
use database::DbRepo;
use scheduler::{error::Error, register::SysTaskTrait, Job};

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

    fn task_cron(&self, sys_id: i32, expression: String) -> Result<Job<DB>, Error> {
        let job =
            Job::new(sys_id, self.db.clone())?.with_cron_job(&expression, |uuid, _jobs| {
                Box::pin(async move {
                    println!("I run async expression demo2 uuid: {uuid}");

                    Ok(())
                })
            })?;

        Ok(job)
    }
}
