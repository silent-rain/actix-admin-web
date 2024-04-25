//! 用户管理

use crate::{
    dto::user::{AddUserReq, GetUserListReq, UpdateUserReq, UpdateUserStatusReq},
    service::user::UserService,
};

use actix_validator::{Json, Query};
use code::Error;
use context::Context;
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};
use tracing::{error, warn};

/// 控制器
pub struct UserController;

impl UserController {
    /// 获取用户列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserListReq>,
    ) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddUserReq>) -> impl Responder {
        let data = data.into_inner();
        // 检查用户
        if data.phone.is_none() && data.email.is_none() {
            error!("请求参数错误, phone/email 不能为空");
            return Response::err(
                Error::InvalidParameterError
                    .into_msg()
                    .with_msg("请求参数错误, phone/email 不能为空"),
            );
        }

        let user_service: UserService = provider.provide();

        let resp = user_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserReq>,
    ) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserStatusReq>,
    ) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.status(*id, data.status.clone() as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}

impl UserController {
    /// 获取用户个人信息
    pub async fn profile(ctx: Context, provider: Data<AInjectProvider>) -> impl Responder {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        let request_id = ctx.get_request_id();
        warn!("profile context request_id: {request_id} user_id: {user_id} username: {username}");

        let user_service: UserService = provider.provide();
        let resp = user_service.profile(user_id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 通过用户ID获角色色列表
    pub async fn roles(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_service: UserService = provider.provide();
        let resp = user_service.roles(*id).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }
}
