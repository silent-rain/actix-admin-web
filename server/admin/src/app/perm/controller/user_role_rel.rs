//! 用户角色关联关系管理

use crate::{
    app::perm::{
        dto::user_role_rel::{AddUserRoleRelReq, GetUserRoleRelListReq},
        service::user_role_rel::UserRoleRelService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 获取用户角色关联列表
    pub async fn list(
        provider: Data<AProvider>,
        req: Query<GetUserRoleRelListReq>,
    ) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 创建用户角色关联
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserRoleRelReq>) -> impl Responder {
        let data = data.into_inner();
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 删除指定的用户角色关联关系
    pub async fn delete(provider: Data<AProvider>, user_id: Path<i32>) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.delete(*user_id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
