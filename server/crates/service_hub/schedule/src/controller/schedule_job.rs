//! 任务调度作业管理

use crate::{
    dto::schedule_job::{
        AddcheduleJobReq, GetScheduleJobReq, UpdatecheduleJobReq, UpdatecheduleJobStatusReq,
    },
    service::schedule_job::ScheduleJobService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct ScheduleJobController;

impl ScheduleJobController {
    /// 获取任务调度列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetScheduleJobReq>,
    ) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取任务调度作业
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加任务调度
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddcheduleJobReq>,
    ) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新任务调度
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdatecheduleJobReq>,
    ) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新任务调度状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdatecheduleJobStatusReq>,
    ) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除任务调度
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
