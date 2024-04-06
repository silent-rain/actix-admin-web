//! 用户管理

use crate::{
    inject::AProvider,
    perm::{
        dto::user::{AddUserReq, GetUserListReq, UpdateUserReq},
        service::user::UserService,
    },
};

use actix_validator::{Json, Query};
use code::Error;
use context::Context;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};
use tracing::warn;

/// 控制器
pub struct UserController;

impl UserController {
    /// 获取用户列表
    pub async fn list(provider: Data<AProvider>, req: Query<GetUserListReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 获取用户信息
    pub async fn info(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 添加用户
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserReq>) -> impl Responder {
        let data = data.into_inner();
        // 检查用户
        if data.phone.is_none() && data.email.is_none() {
            return Response::code(Error::UserAddError);
        }

        let perm_user_service: UserService = provider.provide();

        let resp = perm_user_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户
    pub async fn update(provider: Data<AProvider>, data: Json<UpdateUserReq>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();

        let resp = perm_user_service.update(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 删除用户
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}

impl UserController {
    /// 获取用户个人信息
    pub async fn profile(ctx: Context, provider: Data<AProvider>) -> impl Responder {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        warn!("profile context user_id: {user_id} username: {username}");

        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.profile(user_id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 通过用户ID获取角色列表
    pub async fn roles(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let perm_user_service: UserService = provider.provide();
        let resp = perm_user_service.roles(*id).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }
}
