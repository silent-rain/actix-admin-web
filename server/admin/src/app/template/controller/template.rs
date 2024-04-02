//! 模板管理

use crate::{
    app::template::{
        dto::template::{
            AddAppTemplateStatusReq, AppTemplateInfoReq, AppTemplateListReq,
            BatchDeleteAppTemplateReq, DeleteAppTemplateReq, UpdateAppTemplateReq,
            UpdateAppTemplateStatusReq,
        },
        service::template::AppTemplateService,
    },
    inject::AProvider,
};

use response::Response;

use actix_web::{
    web::{Data, Json, Query},
    Responder,
};

/// 控制器
pub struct AppTemplateController;

impl AppTemplateController {
    /// 获取所有{{InterfaceName}}
    pub async fn all(provider: Data<AProvider>) -> impl Responder {
        let perm_user_service: AppTemplateService = provider.provide();
        let resp = perm_user_service.all().await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取所有{{InterfaceName}}
    pub async fn list(provider: Data<AProvider>, req: Query<AppTemplateListReq>) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.list(req.into_inner()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data_list(results, total)
    }

    /// 获取单个{{InterfaceName}}信息
    pub async fn info(
        provider: Data<AProvider>,
        params: Query<AppTemplateInfoReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.info(params.id).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 添加{{InterfaceName}}
    pub async fn add(
        provider: Data<AProvider>,
        data: Json<AddAppTemplateStatusReq>,
    ) -> impl Responder {
        let data = data.into_inner();
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.add(data).await;
        let result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().data(result)
    }

    /// 更新{{InterfaceName}}
    pub async fn update(
        provider: Data<AProvider>,
        data: Json<UpdateAppTemplateReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.update(data.id, data.status).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }

    /// 更新{{InterfaceName}}状态
    pub async fn status(
        provider: Data<AProvider>,
        data: Json<UpdateAppTemplateStatusReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.status(data.id, data.status).await;
        if let Err(err) = resp {
            return Response::code(err);
        }
        Response::ok().msg("删除成功")
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(
        provider: Data<AProvider>,
        params: Query<DeleteAppTemplateReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.delete(params.id).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(
        provider: Data<AProvider>,
        data: Json<BatchDeleteAppTemplateReq>,
    ) -> impl Responder {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.batch_delete(data.ids.clone()).await;
        let _result = match resp {
            Ok(v) => v,
            Err(err) => return Response::code(err),
        };

        Response::ok().msg("删除成功")
    }
}
