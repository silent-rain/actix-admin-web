//! 登陆

use crate::{
    auth::{
        dto::login::{BrowserInfo, LoginReq},
        LoginService,
    },
    inject::AInjectProvider,
};

use actix_validator::Json;
use response::Response;

use actix_web::{web::Data, HttpRequest, Responder};

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        req: HttpRequest,
        provider: Data<AInjectProvider>,
        data: Json<LoginReq>,
    ) -> impl Responder {
        // Get the remote address from the request
        // let remote_addr = req
        //     .connection_info()
        //     .remote_addr()
        //     .map_or("".to_owned(), |addr| addr.to_string());
        let remote_addr = req
            .peer_addr()
            .map_or("".to_owned(), |addr| addr.ip().to_string());
        // Get the user agent from the request headers
        let user_agent = req
            .headers()
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());
        let browser_info = BrowserInfo {
            remote_addr,
            user_agent,
        };

        let login_service: LoginService = provider.provide();
        let result = login_service.login(browser_info, data.into_inner()).await;
        match result {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }
}
