//! 路由层
pub mod schedule_job;
pub mod schedule_job_event_log;
pub mod schedule_job_status_log;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleRouter;

impl ScheduleRouter {
    /// 注册`调度任务管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/schedule")
            .service(schedule_job::ScheduleJobRouter::admin_register())
            .service(schedule_job_status_log::ScheduleJobStatusLogRouter::admin_register())
            .service(schedule_job_event_log::ScheduleJobEventLogRouter::admin_register())
    }
}
