//! Auto generated OpenAPI documentation
//! utoipa = {version = "4.0", features = ["actix_extras"]}
//! utoipa-swagger-ui = {version = "4.0", features = ["actix-web"]}

use std::sync::Arc;

use actix_web::{get, HttpResponse};
use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        Components, ExternalDocs, Info, OpenApiBuilder, Paths,
    },
    Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::{Config, SwaggerUi};

// 注册 serde_json::Value
// 让 open api 显示为对象
#[derive(Debug, Clone, ToSchema)]
struct Value {}

/// OpenAPI
/// openapi: https://docs.rs/crate/utoipa/4.0.0
#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
    ),
    components(
        schemas(Value),
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tags(
        (name = "actix-admin-api", description = "All about actix",
            external_docs(url = "https://github.com/juhaku/utoipa", description = "Find out more"))
    ),
    external_docs(url = "https://github.com/juhaku/utoipa", description = "More about our APIs")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        )
    }
}

/// 从 /api-docs/openapi.json 文件中读取 OpenAPI 文档
#[get("/api-docs/openapi.json")]
pub async fn openapi_json() -> HttpResponse {
    let file = std::fs::read_to_string("./app/web/api-docs/openapi.json").unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(file)
}

/// swagger ui serve
#[get("/swagger-ui2/{_:.*}")]
pub async fn swagger_ui(tail_path: String) -> HttpResponse {
    // let config = Arc::new(Config::from("/api-docs/openapi.json"));
    let config =
        Arc::new(Config::default().config_url("http://127.0.0.1:8000/api-docs/openapi.json"));
    let swagger_file = match utoipa_swagger_ui::serve(tail_path.as_ref(), config) {
        Ok(swagger_file) => swagger_file,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    swagger_file
        .map(|file| {
            HttpResponse::Ok()
                .content_type(file.content_type)
                .body(file.bytes.to_vec())
        })
        .unwrap_or_else(|| HttpResponse::NotFound().finish())
}

/// 注册内置路由
pub fn register2() -> SwaggerUi {
    let config = Config::default().config_url("http://127.0.0.1:8000/api-docs/openapi.json");
    SwaggerUi::new("/swagger-ui/{_:.*}").config(config).url(
        "/api-docs/openapi.json",
        utoipa::openapi::OpenApi::new(
            utoipa::openapi::Info::new("my application", "0.1.0"),
            utoipa::openapi::Paths::new(),
        ),
    )
}
pub fn register() -> SwaggerUi {
    let external_docs = ExternalDocs::new("http://127.0.0.1:8000/api-docs/openapi.json");
    let openapi = OpenApiBuilder::new()
        .info(Info::new("My api", "1.0.0"))
        .paths(Paths::new())
        .components(Some(Components::new()))
        .external_docs(Some(external_docs))
        .build();

    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi)
}
