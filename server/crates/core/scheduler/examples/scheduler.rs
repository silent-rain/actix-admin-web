//! 定时任务管理
use std::time::Duration;

use database::DbOptions;
use scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "mysql://one:pass@127.0.0.1:3306/actix_admin_web".to_owned();
    let options = DbOptions::default();
    let db = database::Pool::init(database_url.clone(), database_url, options)
        .await
        .expect("初始化数据库失败");

    let mut sched = JobScheduler::new().await?;

    // 任务1
    let job1 = Job::new(1, db.clone())?.with_interval_job(8, |uuid, _jobs| {
        Box::pin(async move {
            println!("I run async every 8 seconds uuid: {uuid} job1");
        })
    })?;
    let job1_uuid = job1.guid().to_string();
    sched.add_job(job1).await?;

    // 任务1重置
    let xjob =
        Job::new(1, db.clone())?.with_cron_uuid(&job1_uuid, "1/5 * * * * *", |uuid, _jobs| {
            Box::pin(async move {
                println!("I run async every 5 seconds uuid: {uuid} job11");
            })
        })?;
    if let Err(err) = sched.add_job(xjob).await {
        println!("=== err: {:#?}", err);
    }

    // 动态添加任务2
    let sched_c = JobScheduler::instance()?;
    let sched2 = JobScheduler::from(sched_c);
    let job2 = Job::new(1, db)?.with_interval_job(5, |uuid, _jobs| {
        Box::pin(async move {
            println!("I run async every 5 seconds uuid: {uuid} job2");
        })
    })?;
    let job2_uuid = job2.guid();
    sched2.add_job(job2.clone()).await?;

    // 即时任务重置
    sched2.remove(&job2_uuid).await?;
    sched2.add_job(job2).await?;

    // 添加要在关闭期间/之后运行的代码
    sched.set_shutdown_handler();

    // 启动调度程序
    sched.start().await?;

    // Wait while the jobs run
    tokio::time::sleep(Duration::from_secs(100)).await;

    // 关闭调度程序
    sched.shutdown().await?;

    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
}
