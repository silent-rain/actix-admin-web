//! 用户信息信息管理

use crate::{
    dto::profile::{AddProfileReq, GetProfilerListReq, UpdateProfileReq, UpdateProfileStatusReq},
    service::profile::ProfileService,
};

use actix_validator::{Json, Query};
use context::Context;
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};
use tracing::warn;

/// 控制器
pub struct ProfileController;

impl ProfileController {
    /// 获取用户信息列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetProfilerListReq>,
    ) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户信息信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户信息
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddProfileReq>) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户信息
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateProfileReq>,
    ) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户信息状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateProfileStatusReq>,
    ) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.status(*id, data.status.clone() as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户信息
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}

impl ProfileController {
    /// 获取用户信息个人信息
    pub async fn profile(ctx: Context, provider: Data<AInjectProvider>) -> impl Responder {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        let request_id = ctx.get_request_id();
        warn!("profile context request_id: {request_id} user_id: {user_id} username: {username}");

        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.profile(user_id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 通过用户信息ID获角色色列表
    pub async fn roles(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let profile_service: ProfileService = provider.provide();
        let resp = profile_service.roles(*id).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }
}
