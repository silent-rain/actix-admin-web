//! 登陆日志

use crate::{
    app::system::{
        dto::user_login::{DisableUserLoginReq, UserLoginInfoReq, UserLoginListReq},
        service::user_login::UserLoginService,
    },
    inject::AProvider,
};

use code::Error;
use entity::sys_user_login;
use response::Response;

use actix_web::{
    web::{Data, Json, Query},
    Responder,
};

/// 控制器
pub struct UserLoginController;

impl UserLoginController {
    /// 查询登陆日志列表
    pub async fn list(provider: Data<AProvider>, req: Query<UserLoginListReq>) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 查询登陆日志详情
    pub async fn info(provider: Data<AProvider>, req: Query<UserLoginInfoReq>) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.info(req.into_inner()).await;
        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::code(Error::DbQueryEmptyError),
        };

        Response::ok().data(result)
    }

    /// 添加登陆日志
    pub async fn add(
        provider: Data<AProvider>,
        data: Json<sys_user_login::Model>,
    ) -> impl Responder {
        let data = data.into_inner();
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }

    /// 禁用登陆日志
    pub async fn disbale_status(
        provider: Data<AProvider>,
        req: Query<DisableUserLoginReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.disbale_status(req.into_inner()).await;
        match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().msg("删除成功")
    }
}
