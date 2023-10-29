//!用户管理

use code::Error;
use database::Pool;
use dto::common::QueryIdReq;
use dto::pagination::Pagination;
use dto::perm_user::AddUserReq;
use response::Response;
use service::user;

use validator::Validate;

/// 服务控制器层
pub struct Controller;

impl Controller {
    pub async fn list(db: &Pool, pagination: &Pagination) -> Response {
        let resp = user::Service::list(db, pagination.page(), pagination.page_size()).await;
        let (results, total) = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data_list(&results, total)
    }

    pub async fn info(db: &Pool, params: QueryIdReq) -> Response {
        let resp = user::Service::info(db, params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::build().code(Error::DbQueryEmptyError),
        };

        Response::build().data(result)
    }

    pub async fn add(db: &Pool, data: AddUserReq) -> Response {
        if let Err(e) = data.validate() {
            return Response::build()
                .code(Error::InvalidParameterError)
                .msg(&e.to_string());
        }

        let resp = user::Service::add(db, data).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };

        Response::build().data(result)
    }
}

pub struct Controller2<'a> {
    db: &'a Pool,
}

impl<'a> Controller2<'a> {
    /// 创建对象
    pub async fn new(db: &'a Pool) -> Controller2<'a> {
        Controller2 { db }
    }

    pub async fn info(&self, params: QueryIdReq) -> Response {
        let resp = user::Service::info(self.db, params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };
        let result = match result {
            Some(v) => v,
            None => return Response::build().code(Error::DbQueryEmptyError),
        };

        Response::build().data(result)
    }

    pub async fn info2(&self, params: QueryIdReq) -> Response {
        let resp = user::Service::info(self.db, params.id).await;

        let result = match resp {
            Ok(v) => v,
            Err(e) => return Response::build().code(e),
        };
        let _result = match result {
            Some(v) => v,
            None => return Response::build().code(Error::DbQueryEmptyError),
        };

        self.info(params).await
    }
}
