//! 定时任务管理
use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;

    // 添加基本cron作业
    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?)
        .await?;

    // 添加异步作业
    let job = Job::new_async("1/7 * * * * *", |uuid, mut l| {
        Box::pin(async move {
            println!("I run async every 7 seconds");

            // Query the next execution time for this job
            let next_tick = l.next_tick_for_job(uuid).await;
            match next_tick {
                Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                _ => println!("Could not get next tick for 7s job"),
            }
        })
    })?;
    let job_id = job.guid();

    sched.add(job).await?;

    // 添加具有给定持续时间的一次性作业
    sched
        .add(Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
            println!("I only run once");
        })?)
        .await?;

    // 创建具有给定持续时间的重复作业，使其可以在之后进行编辑
    let mut jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
        println!("I run repeatedly every 8 seconds");
    })?;

    // 添加作业启动/停止时要执行的操作等。
    jj.on_start_notification_add(
        &sched,
        Box::new(|job_id, notification_id, type_of_notification| {
            Box::pin(async move {
                println!(
                    "Job {:?} was started, notification {:?} ran ({:?})",
                    job_id, notification_id, type_of_notification
                );
            })
        }),
    )
    .await?;

    jj.on_stop_notification_add(
        &sched,
        Box::new(|job_id, notification_id, type_of_notification| {
            Box::pin(async move {
                println!(
                    "Job {:?} was completed, notification {:?} ran ({:?})",
                    job_id, notification_id, type_of_notification
                );
            })
        }),
    )
    .await?;

    jj.on_removed_notification_add(
        &sched,
        Box::new(|job_id, notification_id, type_of_notification| {
            Box::pin(async move {
                println!(
                    "Job {:?} was removed, notification {:?} ran ({:?})",
                    job_id, notification_id, type_of_notification
                );
            })
        }),
    )
    .await?;
    sched.add(jj).await?;

    // Feature 'signal' must be enabled
    // sched.shutdown_on_ctrl_c();

    // 添加要在关闭期间/之后运行的代码
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    // 启动调度程序
    println!("start done");
    sched.start().await?;

    tokio::time::sleep(Duration::from_secs(20)).await;
    sched.remove(&job_id).await?;
    // sched.add(job)

    // Wait while the jobs run
    tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}
