//! Api 操作日志中间件
use std::{
    future::{ready, Ready},
    pin::Pin,
    rc::Rc,
};

use code::ErrorMsg;
use context::Context;
use response::Response;
use service_hub::{
    inject::AInjectProvider,
    log::{dto::api_operation::AddApiOperationReq, enums::HttpType, ApiOperationService},
    system::constant::HEADERS_X_IMG,
};

use actix_http::h1::Payload;
use actix_web::{
    body::{to_bytes, BoxBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web::{BytesMut, Data},
    Error, HttpMessage, HttpRequest,
};
use futures::{Future, StreamExt};
use tracing::error;

/// Api 操作日志中间件
#[derive(Default)]
pub struct ApiOperation {}

impl<S> Transform<S, ServiceRequest> for ApiOperation
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = ApiOperationMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiOperationMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct ApiOperationMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for ApiOperationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let start_time = std::time::Instant::now(); // 请求开始的时间

        let service = Rc::clone(&self.service);

        let provider = match req.app_data::<Data<AInjectProvider>>() {
            Some(v) => v.as_ref().clone(),
            None => {
                return Box::pin(async move {
                    error!("获取服务实例失败");
                    Err(Response::code(code::Error::InjectAproviderObj).into())
                })
            }
        };

        let inner_req = req.request();

        // 解析请求信息
        let mut data = Self::parse_req(inner_req);
        let content_type = req.content_type().to_uppercase();
        Box::pin(async move {
            let mut request_body = BytesMut::new();
            if content_type != "multipart/form-data".to_uppercase() {
                // EXTRACT THE BODY OF REQUES
                while let Some(chunk) = req.take_payload().next().await {
                    request_body.extend_from_slice(&chunk?);
                }

                // 重新设置body
                let (_, mut orig_payload) = Payload::create(true);
                orig_payload.unread_data(request_body.clone().freeze());
                req.set_payload(actix_http::Payload::from(orig_payload));
            }

            // 添加请求操作日志
            data.cost = start_time.elapsed().as_millis() as u64;
            let body = Self::get_request_body(&request_body)
                .map_or("body data parsing error ".to_string(), |v| v);
            data.body = Some(body);
            if let Err(err) = Self::add_api_operation_log(provider.clone(), data.clone()).await {
                return Err(Response::err(err).into());
            }

            // 响应
            let mut fut = service.call(req).await?;
            let mut body = "".to_owned();
            // 图片body数据不入库
            if fut
                .response_mut()
                .headers_mut()
                .get(HEADERS_X_IMG)
                .is_some()
            {
                fut.response_mut().headers_mut().remove(HEADERS_X_IMG);
            } else {
                (fut, body) = Self::response_manipulate_body(fut).await;
            }

            // 添加响应操作日志
            data.cost = start_time.elapsed().as_millis() as u64;
            data.http_type = HttpType::Rsp;
            // TODO 添加字符限制, 如果太大则进行省略
            data.body = Some(body);
            data.status_code = fut.status().as_u16() as i32;

            if let Err(err) = Self::add_api_operation_log(provider.clone(), data).await {
                return Err(Response::err(err).into());
            }

            Ok(fut)
        })
    }
}

impl<S> ApiOperationMiddlewareService<S> {
    /// 获取请求体的 body
    fn get_request_body(request_body: &BytesMut) -> Result<String, ErrorMsg> {
        // 解析字符串为serde_json::Value
        let json_str = String::from_utf8_lossy(request_body).to_string();

        if json_str.is_empty() {
            return Ok("".to_owned());
        }

        let data: serde_json::Value = serde_json::from_str(&json_str).map_err(|err| {
            error!("body 数据转换错误, err: {err}");
            code::Error::JsonConvert
                .into_msg()
                .with_msg("body 数据转换错误")
        })?;

        // 将Value转换为紧凑格式的字符串
        let body = serde_json::to_string(&data).map_err(|err| {
            error!("body 数据解析错误, err: {err}");
            code::Error::JsonConvert
                .into_msg()
                .with_msg("body 数据解析错误")
        })?;

        Ok(body)
    }

    /// 响应体
    async fn response_manipulate_body(res: ServiceResponse) -> (ServiceResponse, String) {
        let (req, res) = res.into_parts();

        let (res, body) = res.into_parts();
        // TODO body 最大值限制, 防止日志刺穿
        let body_bytes = to_bytes(body).await.unwrap();
        let body = String::from_utf8_lossy(&body_bytes).to_string();

        let res = res.set_body(BoxBody::new(body.clone()));
        let service = ServiceResponse::new(req, res);

        (service, body)
    }

    /// 解析请求信息
    fn parse_req(req: &HttpRequest) -> AddApiOperationReq {
        // 添加上下文
        let (user_id, username, request_id) = match req.extensions_mut().get::<Context>() {
            Some(ctx) => (
                Some(ctx.get_user_id()),
                Some(ctx.get_user_name()),
                Some(ctx.get_request_id()),
            ),
            None => (None, None, None),
        };

        let status_code = StatusCode::OK.as_u16() as i32; // 默认请求成功
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query = req.query_string().to_owned();
        let remote_addr = req
            .peer_addr()
            .map_or("".to_owned(), |addr| addr.ip().to_string());
        // Get the user agent from the request headers
        let user_agent = req
            .headers()
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());

        // TODO 调研中
        let match_pattern = req.match_pattern();
        println!("match_pattern: {:?}", match_pattern);

        AddApiOperationReq {
            user_id,
            username,
            request_id,
            status_code,
            method,
            path,
            query: Some(query),
            body: None,
            remote_addr,
            user_agent,
            cost: 0,
            http_type: HttpType::Req,
            note: None,
        }
    }

    /// 添加操作日志
    async fn add_api_operation_log(
        provider: AInjectProvider,
        data: AddApiOperationReq,
    ) -> Result<(), code::ErrorMsg> {
        let api_operation_service: ApiOperationService = provider.provide();
        let _user = api_operation_service.add(data).await?;
        Ok(())
    }
}
