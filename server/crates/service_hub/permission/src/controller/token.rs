//! 令牌管理

use crate::{
    dto::token::{AddTokenReq, GetTokenListReq, UpdateTokenReq, UpdateTokenStatusReq},
    service::token::TokenService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct TokenController;

impl TokenController {
    /// 获取令牌列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetTokenListReq>,
    ) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取令牌信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加令牌
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddTokenReq>) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新令牌
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateTokenReq>,
    ) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新令牌状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateTokenStatusReq>,
    ) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.status(*id, data.status.clone() as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除令牌
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let token_service: TokenService = provider.provide();
        let resp = token_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
