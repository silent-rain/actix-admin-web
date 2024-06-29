//! 会员等级管理

use crate::{
    dto::member_level::{
        AddMemberLevelReq, GetMemberLevelListReq, UpdateMemberLevelReq, UpdateMemberLevelStatusReq,
    },
    service::member_level::MemberLevelService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct MemberLevelController;

impl MemberLevelController {
    /// 获取会员等级列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetMemberLevelListReq>,
    ) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取会员等级信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加会员等级
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddMemberLevelReq>,
    ) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新会员等级
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateMemberLevelReq>,
    ) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新会员等级状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateMemberLevelStatusReq>,
    ) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除会员等级
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let member_level_service: MemberLevelService = provider.provide();
        let resp = member_level_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
