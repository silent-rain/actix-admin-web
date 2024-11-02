use std::sync::Arc;

use database::{mock::Mock, DbRepo};
use migration::Migrator;
use response::Response;

use actix_http::{Request, StatusCode};
use actix_web::{
    body::to_bytes,
    dev::{Service, ServiceResponse},
    test, web, App, Scope,
};
use inject::InjectProvider;
use sea_orm_migration::{migrator::MigratorTrait, MigrationTrait, SchemaManager};
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

pub struct MockRequest<F>
where
    F: Fn() -> Scope + 'static,
{
    pool: Arc<dyn DbRepo>,
    routes: F,
}

impl<F> MockRequest<F>
where
    F: Fn() -> Scope + 'static,
{
    pub async fn new(routes: F) -> Self {
        let pool = Mock::connect().await;
        MockRequest { pool, routes }
    }

    /// 是否启用日志
    pub fn enabled_log(self, enabled: bool) -> Self {
        if !enabled {
            return self;
        }
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .with_level(true)
            .with_line_number(true)
            .init();

        self
    }

    /// 迁移库表列表
    pub async fn migrations(self, migrations: Vec<&dyn MigrationTrait>) -> Result<Self, Error> {
        for migration in migrations {
            let manager = SchemaManager::new(self.pool.wdb());
            migration
                .up(&manager)
                .await
                .map_err(|err| Error::DbInit(err.to_string()))?;
        }
        Ok(self)
    }

    /// 迁移所有库表
    pub async fn all_migrations(self) -> Result<Self, Error> {
        Migrator::up(self.pool.wdb(), None)
            .await
            .map_err(|err| Error::DbInit(err.to_string()))?;
        Ok(self)
    }

    /// 创建一个测试服务器
    async fn test_service(
        &self,
    ) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        let provider = InjectProvider::anew(self.pool.clone());

        test::init_service(
            App::new()
                .app_data(web::Data::new(provider.clone()))
                .service((self.routes)()),
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
    pub async fn get<T: Serialize>(
        &self,
        route: &str,
        params: T,
    ) -> Result<ServiceResponse, Error> {
        let app = self.test_service().await;
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
    pub async fn assert_get<T: Serialize>(&self, route: &str, data: T) -> Result<Response, Error> {
        let response = self.get(route, data).await?;
        if response.status() != StatusCode::OK {
            error!(
                "response status: {:#?}, data: {:#?}",
                response.status(),
                response
            );

            error!("match_info: {:#?}", response.request().match_info());
        }
        assert!(response.status().is_success());
        let body: Response = Self::json(response).await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::Responder;

    pub async fn health() -> impl Responder {
        Response::ok().data("ok")
    }

    #[tokio::test]
    async fn test_demo() -> Result<(), Error> {
        let response = MockRequest::new(|| {
            web::scope("/api/v1/admin")
                .service(web::scope("/template/app-templates").route("/all", web::get().to(health)))
        })
        .await
        .enabled_log(true)
        .assert_get("/api/v1/admin/template/app-templates/all", ())
        .await?;
        println!("response: {:#?}", response);

        Ok(())
    }
}
