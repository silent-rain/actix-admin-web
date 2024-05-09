//! 任务调度事件日志管理

use crate::controller::schedule_event_log::ScheduleEventLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleEventLogRouter;

impl ScheduleEventLogRouter {
    /// 注册`任务调度事件日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/event-logs")
            .route("", web::get().to(ScheduleEventLogController::list))
            .route("/{id}", web::get().to(ScheduleEventLogController::info))
        // .route("", web::post().to(ScheduleJobLogController::add))
        // .route("/{id}", web::delete().to(ScheduleJobLogController::delete))
    }
}
