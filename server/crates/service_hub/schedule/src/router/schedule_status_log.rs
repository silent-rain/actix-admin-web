//! 任务调度状态日志管理

use crate::controller::schedule_status_log::ScheduleStatusLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleStatusLogRouter;

impl ScheduleStatusLogRouter {
    /// 注册`任务调度状态日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/event-logs")
            .route("", web::get().to(ScheduleStatusLogController::list))
            .route("/{id}", web::get().to(ScheduleStatusLogController::info))
        // .route("", web::post().to(ScheduleJobLogController::add))
        // .route("/{id}", web::delete().to(ScheduleJobLogController::delete))
    }
}
