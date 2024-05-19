use crate::router;

use database::mock::Mock;
use migration::m20230210_145453_create_app_template::Migration;
use migration::Migrator;
use response::Response;

use actix_http::{Request, StatusCode};
use actix_web::{
    body::to_bytes,
    dev::{Service, ServiceResponse},
    test::{self},
    web, App,
};
use inject::InjectProvider;
use sea_orm_migration::migrator::MigratorTrait;
use serde::Serialize;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("init db failed, {0}")]
    DbInit(String),
    #[error("BoxBody To Bytes Error, {0}")]
    BoxBodyToBytes(String),
    #[error("Deserialize Bytes Error, {0}")]
    DeserializeBytes(String),
}

#[derive(Debug, Default)]
pub struct MockRequest {}

impl MockRequest {
    /// 创建一个测试服务器
    async fn test_service(
    ) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .with_level(true)
            .with_line_number(true)
            .init();

        // 初始化单个表，并返回db
        // use migration::m20230210_145453_create_app_template::Migration;
        let pool = Mock::from_migration(&Migration)
            .await
            .map_err(|err| Error::DbInit(err.to_string()))
            .expect("init mock db failed");

        // 初始化db
        // let pool = Mock::connect().await;
        // 库表迁移器
        if let Err(err) = Migrator::up(pool.wdb(), None).await {
            panic!("表迁移失败. err: {err}");
        }

        let provider = InjectProvider::anew(pool.into());

        test::init_service(
            App::new()
                .app_data(web::Data::new(provider.clone()))
                .service(router::register()),
        )
        .await
    }

    /// 返回Json响应体
    pub async fn json<T>(res: ServiceResponse) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let body = res.into_body();

        let body_bytes = to_bytes(body)
            .await
            .map_err(|err| Error::BoxBodyToBytes(err.to_string()))?;

        let target: T = serde_json::from_slice(&body_bytes)
            .map_err(|err| Error::DeserializeBytes(err.to_string()))?;

        Ok(target)
    }

    /// Get 请求
    pub async fn get<T: Serialize>(route: &str, params: T) -> Result<ServiceResponse, Error> {
        let app = Self::test_service().await;
        let resp = test::call_service(
            &app,
            test::TestRequest::get()
                // .cookie()
                .uri(route)
                .set_form(params)
                .to_request(),
        )
        .await;

        Ok(resp)
    }

    /// Get 请求并判断请求是否成功
    pub async fn assert_get<T: Serialize>(route: &str, data: T) -> Result<Response, Error> {
        let response = Self::get(route, data).await?;
        if response.status() != StatusCode::OK {
            error!(
                "response status: {:#?}, data: {:#?}",
                response.status(),
                response
            );
        }
        assert!(response.status().is_success());
        let body: Response = Self::json(response).await.expect("xxx");
        Ok(body)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASE_URL: &str = "/api/v1/admin";

    #[tokio::test]
    async fn test_demo() -> Result<(), Error> {
        let response =
            MockRequest::assert_get(&format!("{BASE_URL}/template/app-templates/all",), ()).await?;
        println!("response: {:#?}", response);

        Ok(())
    }
}
