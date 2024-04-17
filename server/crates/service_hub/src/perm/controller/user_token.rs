//! 用户Token令牌管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::user_token::{
            AddUserTokenReq, GetUserTokenListReq, UpdateUserTokenReq, UpdateUserTokenStatusReq,
        },
        service::user_token::UserTokenService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct UserTokenController;

impl UserTokenController {
    /// 获取用户令牌列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserTokenListReq>,
    ) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户令牌信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户令牌
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddUserTokenReq>,
    ) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户令牌
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserTokenReq>,
    ) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户令牌状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserTokenStatusReq>,
    ) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户令牌
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_token_service: UserTokenService = provider.provide();
        let resp = user_token_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
