//! 用户管理

use crate::state::AppState;
use code::Error;
use dto::perm_user::AddUserReq;
use dto::{common::QueryIdReq, pagination::Pagination};
use response::Response;
use service::user;

use actix_web::{get, web, Responder};
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/v1/user/list",
    params(
    ),
    responses(
        (status = 200, description = "succesfully", body=Response),
    ),
)]
#[get("/user/list")]
pub async fn list(state: web::Data<AppState>, page: web::Query<Pagination>) -> impl Responder {
    let resp = user::Service::list(&state.db, page.into_inner()).await;
    let (results, total) = match resp {
        Ok(v) => v,
        Err(e) => return Response::build().code(e),
    };

    Response::build().data_list(results, total)
}

#[utoipa::path(
    get,
    path = "/api/v1/user/info",
    params(
    ),
    responses(
        (status = 200, description = "succesfully", body=Response),
    ),
)]
#[get("/user/info")]
pub async fn info(state: web::Data<AppState>, params: web::Query<QueryIdReq>) -> impl Responder {
    let resp = user::Service::info(&state.db, params.id).await;

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

#[utoipa::path(
    get,
    path = "/api/v1/user/add",
    params(
    ),
    responses(
        (status = 200, description = "succesfully", body=Response),
    ),
)]
#[get("/user/add")]
pub async fn add(state: web::Data<AppState>, data: web::Json<AddUserReq>) -> impl Responder {
    let data = data.into_inner();
    if let Err(e) = data.validate() {
        return Response::build()
            .code(Error::InvalidParameterError)
            .msg(&e.to_string());
    }

    let resp = user::Service::add(&state.db, data).await;

    let result = match resp {
        Ok(v) => v,
        Err(e) => return Response::build().code(e),
    };

    Response::build().data(result)
}
