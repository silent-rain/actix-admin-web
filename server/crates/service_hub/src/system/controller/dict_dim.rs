//! 字典维度管理

use crate::{
    inject::AInjectProvider,
    system::{
        dto::dict_dim::{
            AddDictDimReq, GetDictDimListReq, UpdateDictDimReq, UpdateDictDimStatusReq,
        },
        service::dict_dim::DictDimService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct DictDimController;

impl DictDimController {
    /// 获取字典维度列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetDictDimListReq>,
    ) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取字典维度信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加字典维度
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddDictDimReq>) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典维度
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDimReq>,
    ) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新字典维度状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateDictDimStatusReq>,
    ) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除字典维度
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let role_service: DictDimService = provider.provide();
        let resp = role_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
