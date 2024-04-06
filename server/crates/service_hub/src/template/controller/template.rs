//! 模板管理

use crate::{
    inject::AProvider,
    template::{
        dto::template::{
            AddAppTemplateReq, AppTemplateInfoReq, AppTemplateListReq, BatchDeleteAppTemplateReq,
            UpdateAppTemplateReq, UpdateAppTemplateStatusReq,
        },
        service::template::AppTemplateService,
    },
};

use response::Response;

use actix_web::{
    web::{Data, Json, Path, Query},
    Responder,
};

/// 控制器
pub struct AppTemplateController;

impl AppTemplateController {
    /// 获取所有{{InterfaceName}}
    pub async fn all(provider: Data<AProvider>) -> impl Responder {
        let perm_user_service: AppTemplateService = provider.provide();
        let resp = perm_user_service.all().await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 获取所有{{InterfaceName}}
    pub async fn list(provider: Data<AProvider>, req: Query<AppTemplateListReq>) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::code(err),
        }
    }

    /// 获取单个{{InterfaceName}}信息
    pub async fn info(
        provider: Data<AProvider>,
        params: Query<AppTemplateInfoReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.info(params.id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::code(err),
        }
    }

    /// 添加{{InterfaceName}}
    pub async fn add(provider: Data<AProvider>, data: Json<AddAppTemplateReq>) -> impl Responder {
        let data = data.into_inner();
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.add(data).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 更新{{InterfaceName}}
    pub async fn update(
        provider: Data<AProvider>,
        data: Json<UpdateAppTemplateReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.update(data.id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 更新{{InterfaceName}}状态
    pub async fn status(
        provider: Data<AProvider>,
        data: Json<UpdateAppTemplateStatusReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.status(data.id, data.status).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(provider: Data<AProvider>, id: Path<i32>) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(
        provider: Data<AProvider>,
        data: Json<BatchDeleteAppTemplateReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.batch_delete(data.ids.clone()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::code(err),
        }
    }
}
