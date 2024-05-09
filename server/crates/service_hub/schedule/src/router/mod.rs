//! 路由层
pub mod schedule_event_log;
pub mod schedule_job;
pub mod schedule_status_log;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleRouter;

impl ScheduleRouter {
    /// 注册`任务调度作业管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/schedule")
            .service(schedule_job::ScheduleJobRouter::admin_register())
            .service(schedule_status_log::ScheduleStatusLogRouter::admin_register())
            .service(schedule_event_log::ScheduleEventLogRouter::admin_register())
    }
}
