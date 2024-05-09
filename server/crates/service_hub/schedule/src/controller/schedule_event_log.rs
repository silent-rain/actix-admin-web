//! 任务调度事件日志管理

use crate::{
    dto::schedule_event_log::{AddScheduleEventLogReq, GetScheduleEventLogListReq},
    service::schedule_event_log::ScheduleEventLogService,
};

use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct ScheduleEventLogController;

impl ScheduleEventLogController {
    /// 获取任务调度事件日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetScheduleEventLogListReq>,
    ) -> impl Responder {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let resp = schedule_event_log_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取任务调度事件日志的详细信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let resp = schedule_event_log_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加任务调度事件日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddScheduleEventLogReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let resp = schedule_event_log_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除任务调度事件日志
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let resp = schedule_event_log_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
