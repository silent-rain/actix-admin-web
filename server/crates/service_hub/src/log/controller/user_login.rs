//! 登陆日志

use crate::{
    inject::AInjectProvider,
    log::{
        dto::user_login::{
            AddUserLoginInfoReq, GetUserLoginListReq, UpdateUserLoginDisabledStatusReq,
            UpdateUserLoginStatusReq,
        },
        service::user_login::UserLoginService,
    },
};

use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct UserLoginController;

impl UserLoginController {
    /// 获取登录日志列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserLoginListReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取登录日志信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加登陆日志
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddUserLoginInfoReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新登录日志状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserLoginStatusReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.status(*id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新登录日志禁用状态
    pub async fn disabled(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserLoginDisabledStatusReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.disabled(*id, data.disabled).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
