//! 部门管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::dept::{AddDeptReq, GetDeptListReq, UpdateDeptReq, UpdateDeptStatusReq},
        service::dept::DeptService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct DeptController;

impl DeptController {
    /// 获取部门列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDeptListReq>,
    ) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取部门信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加部门
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddDeptReq>) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新部门
    pub async fn update(
        provider: Data<AInjectProvider>,
        data: Json<UpdateDeptReq>,
    ) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service.update(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新部门状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        data: Json<UpdateDeptStatusReq>,
    ) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service
            .status(data.id, data.status.clone().into())
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除部门
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let dept_service: DeptService = provider.provide();
        let resp = dept_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
