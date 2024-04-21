//! 定时任务管理
use std::time::Duration;

use scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sched = JobScheduler::new().await?;

    // 任务1
    let job1 = Job::interval_job(8)?;
    let job1_uuid = job1.guid().to_string();
    sched.add_job(job1).await?;

    // 任务1重置
    let xjob = Job::form_cron_uuid(&job1_uuid, "1/5 * * * * *")?;
    if let Err(err) = sched.add_job(xjob).await {
        println!("=== err: {:#?}", err);
    }

    // 动态添加任务2
    // let sched_c = JobScheduler::instance()?;
    // let sched2 = JobScheduler::from(sched_c);
    // let job2 = Job::interval_job(5)?;
    // sched2.add_job(job2).await?;

    // 即时任务重置
    // sched2.remove(job_id2)?;
    // sched2.add_job(job2).await?;

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
