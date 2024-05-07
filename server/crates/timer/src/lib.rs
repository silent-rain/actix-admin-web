//! 系统内置定时任务

mod demo;
mod demo2;

use database::DbRepo;
use scheduler::{
    error::Error,
    register::{SysTaskRegister, UserTaskRegister},
    JobScheduler,
};
use tokio::runtime::Handle;

/// 任务注册
pub struct TimerRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    sys_task: SysTaskRegister<DB>,
    user_task: UserTaskRegister<DB>,
    db: DB,
}

impl<DB> TimerRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        TimerRegister {
            sys_task: SysTaskRegister::new(db.clone()),
            user_task: UserTaskRegister::new(db.clone()),
            db,
        }
    }

    /// 统一添加系统任务的位置
    fn register(&mut self) {
        self.sys_task
            .add_task(Box::new(demo::DemoTask::new(self.db.clone())));
        self.sys_task
            .add_task(Box::new(demo2::DemoTask2::new(self.db.clone())));
    }

    /// 任务初始化
    pub async fn init(&mut self) -> Result<(), Error> {
        // 添加系统任务
        self.register();

        // 任务注册
        self.sys_task.register().await?;
        self.user_task.register().await?;

        // 开始执行任务调度程序
        let mut sched = JobScheduler::new().await?;
        sched.start().await?;
        sched.set_shutdown_handler();
        Ok(())
    }
}

impl<DB> TimerRegister<DB>
where
    DB: DbRepo + Clone + Send + Sync + 'static,
{
    pub fn start(db: DB) {
        let handle = Handle::current();
        // 创建一个新的操作系统线程来运行异步代码
        std::thread::spawn(move || {
            // 使用Tokio Runtime的handle运行异步代码
            handle.block_on(async {
                let mut task = TimerRegister::new(db);
                task.init().await.expect("定时任务初始化失败");
            });
        });
    }
}

/// 关闭调度程序
pub struct TimerShutdown;

impl TimerShutdown {
    /// 关闭调度程序
    pub async fn shutdown() -> Result<(), Error> {
        JobScheduler::new().await?.shutdown().await
    }
}
