//! 定时任务示例
use database::DbRepo;
use scheduler::{error::Error, register::SysTaskTrait, Job};

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
        "task_demo1".to_owned()
    }

    fn task_interval(&self, sys_id: i32, interval: i32) -> Result<Job<DB>, Error> {
        let job = Job::new(sys_id, self.db.clone())?.with_interval_job(
            interval as u64,
            |uuid, _jobs| {
                Box::pin(async move {
                    println!("I run async interval demo1 uuid: {uuid}");
                    Ok(())
                })
            },
        )?;

        Ok(job)
    }
}
