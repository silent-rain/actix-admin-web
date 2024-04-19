//! 定时任务管理

use crate::{
    inject::AInjectProvider,
    schedule::{
        dto::schedule_job::{
            AddcheduleJobReq, GetScheduleJobReq, UpdatecheduleJobReq, UpdatecheduleJobStatusReq,
        },
        service::schedule_job::ScheduleJobService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct ScheduleJobController;

impl ScheduleJobController {
    /// 获取定时任务列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetScheduleJobReq>,
    ) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取定时任务信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加定时任务
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

    /// 更新定时任务
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

    /// 更新定时任务状态
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

    /// 删除定时任务
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let resp = schedule_job_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
