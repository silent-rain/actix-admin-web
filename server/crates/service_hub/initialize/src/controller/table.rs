//! 库表初始化

use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Json},
    Responder,
};

use crate::{dto::table::AddAdminUserReq, service::table::TableService};

/// 控制器
pub struct TableController;

impl TableController {
    /// 初始化库表
    pub async fn table(
        provider: Data<AInjectProvider>,
        data: Json<AddAdminUserReq>,
    ) -> impl Responder {
        let table_service: TableService = provider.provide();
        let resp = table_service.table(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok().with_msg("初始化成功"),
            Err(err) => Response::err(err),
        }
    }
}
