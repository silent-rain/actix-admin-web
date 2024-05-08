//! 任务调度状态日志管理

use crate::ScheduleJobStatusLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleJobStatusLogRouter;

impl ScheduleJobStatusLogRouter {
    /// 注册`任务调度状态日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/job-event-logs")
            .route("", web::get().to(ScheduleJobStatusLogController::list))
            .route("/{id}", web::get().to(ScheduleJobStatusLogController::info))
        // .route("", web::post().to(ScheduleJobLogController::add))
        // .route("/{id}", web::delete().to(ScheduleJobLogController::delete))
    }
}
