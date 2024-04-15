//! 定时任务

use crate::schedule::ScheduleJobLogController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleJobLogRouter;

impl ScheduleJobLogRouter {
    /// 注册`定时任务日志管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/job-logs")
            .route("", web::get().to(ScheduleJobLogController::list))
            .route("/{id}", web::get().to(ScheduleJobLogController::info))
        // .route("", web::post().to(ScheduleJobLogController::add))
        // .route("/{id}", web::delete().to(ScheduleJobLogController::delete))
    }
}
