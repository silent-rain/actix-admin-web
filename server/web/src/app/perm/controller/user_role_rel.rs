//! 用户角色关联关系管理

use crate::{
    app::perm::{
        dto::user_role_rel::{AddUserRoleRelReq, DeleteUserRoleRelReq, GetUserRoleRelListReq},
        service::user_role_rel::UserRoleRelService,
    },
    inject::AProvider,
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{web::Data, Responder};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 角色列表查询
    pub async fn list(
        provider: Data<AProvider>,
        req: Query<GetUserRoleRelListReq>,
    ) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data_list(results, total)
    }

    /// 添加角色信息
    pub async fn add(provider: Data<AProvider>, data: Json<AddUserRoleRelReq>) -> impl Responder {
        let data = data.into_inner();
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.add(data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().data(result)
    }

    /// 删除关联关系
    pub async fn delete(
        provider: Data<AProvider>,
        req: Query<DeleteUserRoleRelReq>,
    ) -> impl Responder {
        let perm_user_service: UserRoleRelService = provider.provide();
        let resp = perm_user_service.delete(req.into_inner()).await;
        let _result = match resp {
            Ok(v) => v,
            Err(e) => return Response::code(e),
        };

        Response::ok().msg("删除成功")
    }
}
