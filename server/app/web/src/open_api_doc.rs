//! Auto generated OpenAPI documentation
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

use crate::routes;
use response::Response;

// use routes::user::list;

// 注册 serde_json::Value
// 让 open api 显示为对象
#[derive(Debug, Clone, ToSchema)]
struct Value {}

/// 在运行时修改OpenAPI
/// openapi: https://docs.rs/crate/utoipa/4.0.0
#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
        // 打招呼
        routes::welcome::greet,
        // 用户管理
        routes::user::list,
        routes::user::info,
        routes::user::add,
        // 注册、登录
        // routes::system::user::register_user,
        // routes::system::user::login,
        // // token 管理
        // routes::system::user_token::get_all_token,
        // routes::system::user_token::get_token_info,
        // routes::system::user_token::add_token,
        // routes::system::user_token::update_token,
        // routes::system::user_token::delete_token,
        // // token API 管理
        // routes::system::token_api_auth::get_all_token_uri,
        // routes::system::token_api_auth::get_token_uri_list,
        // routes::system::token_api_auth::get_token_uri_info,
        // routes::system::token_api_auth::add_token_uri,
        // routes::system::token_api_auth::update_token_uri_status,
        // routes::system::token_api_auth::update_token_uri_expire,
        // routes::system::token_api_auth::delete_token_uri,
    ),
    components(
        schemas(Value),
        schemas(Response),
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
