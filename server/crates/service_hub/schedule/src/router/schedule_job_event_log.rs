//! 任务调度事件日志管理

use crate::ScheduleJobEventLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleJobEventLogRouter;

impl ScheduleJobEventLogRouter {
    /// 注册`任务调度事件日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/job-event-logs")
            .route("", web::get().to(ScheduleJobEventLogController::list))
            .route("/{id}", web::get().to(ScheduleJobEventLogController::info))
        // .route("", web::post().to(ScheduleJobLogController::add))
        // .route("/{id}", web::delete().to(ScheduleJobLogController::delete))
    }
}
