//! 任务调度状态日志管理

use crate::{
    dto::schedule_job_status_log::{
        AddScheduleJobStatusLogReq, GetScheduleJobStatusListLogReq, UpdateScheduleJobStatusLogReq,
        UpdateScheduleJobStatusLogSatausReq,
    },
    service::schedule_job_status_log::ScheduleJobStatusLogService,
};

use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct ScheduleJobStatusLogController;

impl ScheduleJobStatusLogController {
    /// 获取任务调度状态日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetScheduleJobStatusListLogReq>,
    ) -> impl Responder {
        let schedule_job_status_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_status_log_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取任务调度状态日志的详细信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_status_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_status_log_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加任务调度状态日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddScheduleJobStatusLogReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let schedule_job_status_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_status_log_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新任务调度状态日志
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateScheduleJobStatusLogReq>,
    ) -> impl Responder {
        let schedule_job_event_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_event_log_service
            .update(*id, data.into_inner())
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新任务调度状态日志
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateScheduleJobStatusLogSatausReq>,
    ) -> impl Responder {
        let schedule_job_event_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_event_log_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除任务调度状态日志
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_status_log_service: ScheduleJobStatusLogService = provider.provide();
        let resp = schedule_job_status_log_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
