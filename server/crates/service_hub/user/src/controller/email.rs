//! 用户邮箱管理

use crate::{
    dto::email::{AddEmailReq, GetEmailListReq, UpdateEmailReq},
    service::email::EmailService,
};

use actix_validator::{Json, Query};
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct EmailController;

impl EmailController {
    /// 获取用户邮箱列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetEmailListReq>,
    ) -> impl Responder {
        let email_service: EmailService = provider.provide();
        let resp = email_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户邮箱信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let email_service: EmailService = provider.provide();
        let resp = email_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户邮箱
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddEmailReq>) -> impl Responder {
        let email_service: EmailService = provider.provide();
        let resp = email_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户邮箱
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateEmailReq>,
    ) -> impl Responder {
        let email_service: EmailService = provider.provide();
        let resp = email_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户邮箱
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let email_service: EmailService = provider.provide();
        let resp = email_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
