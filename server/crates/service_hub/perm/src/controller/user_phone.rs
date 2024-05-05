//! 用户手机号管理

use crate::{
    dto::user_phone::{AddUserPhoneReq, GetUserPhoneListReq, UpdateUserPhoneReq},
    service::user_phone::UserPhoneService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct UserPhoneController;

impl UserPhoneController {
    /// 获取用户手机号列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserPhoneListReq>,
    ) -> impl Responder {
        let user_phone_service: UserPhoneService = provider.provide();
        let resp = user_phone_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户手机号信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_phone_service: UserPhoneService = provider.provide();
        let resp = user_phone_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户手机号
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddUserPhoneReq>,
    ) -> impl Responder {
        let user_phone_service: UserPhoneService = provider.provide();
        let resp = user_phone_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户手机号
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserPhoneReq>,
    ) -> impl Responder {
        let user_phone_service: UserPhoneService = provider.provide();
        let resp = user_phone_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户手机号
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_phone_service: UserPhoneService = provider.provide();
        let resp = user_phone_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
