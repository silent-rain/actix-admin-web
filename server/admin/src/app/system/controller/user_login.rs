//! 登陆日志

use crate::{
    app::system::{
        dto::user_login::{UserLoginInfoReq, UserLoginListReq, UserLoginStatusReq},
        service::user_login::UserLoginService,
    },
    inject::AProvider,
};

use entity::sys_user_login;
use response::Response;

use actix_web::{
    web::{Data, Json, Query},
    Responder,
};

/// 控制器
pub struct UserLoginController;

impl UserLoginController {
    /// 获取登录日志列表
    pub async fn list(provider: Data<AProvider>, req: Query<UserLoginListReq>) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取登录日志信息
    pub async fn info(provider: Data<AProvider>, req: Query<UserLoginInfoReq>) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.info(req.into_inner()).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
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
        let data: sys_user_login::ActiveModel = data.into();
        let resp = user_login_service.add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 更新登录日志状态
    pub async fn status(
        provider: Data<AProvider>,
        req: Query<UserLoginStatusReq>,
    ) -> impl Responder {
        let user_login_service: UserLoginService = provider.provide();
        let resp = user_login_service.status(req.into_inner()).await;
        match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
