//! 职级管理

use crate::{
    dto::rank::{AddRankReq, GetRankListReq, UpdateRankReq, UpdateRankStatusReq},
    service::rank::RankService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct RankController;

impl RankController {
    /// 获取职级列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetRankListReq>,
    ) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取职级信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加职级
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddRankReq>) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新职级
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateRankReq>,
    ) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新职级状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateRankStatusReq>,
    ) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除职级
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let department_service: RankService = provider.provide();
        let resp = department_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
