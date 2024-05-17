use database::mock::Mock;
use inject::InjectProvider;
use template::TemplateRouter;

use actix_web::{dev::ServiceResponse, test, web, App};

async fn get(route: &str) -> ServiceResponse {
    let pool = Mock::connect().await;

    let provider = InjectProvider::anew(pool.into());

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(provider.clone()))
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/admin").service(TemplateRouter::admin_register())),
            ),
    )
    .await;

    test::call_service(
        &mut app,
        test::TestRequest::get()
            // .cookie(cookie.clone())
            .uri(route)
            .to_request(),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo() {
        let response = get("/api/v1/admin/template/app-templates").await;
        println!("==== {:#?}", response);
        println!("==== {:#?}", response.request().match_info());
        println!("==== {:#?}", response.status());
        assert!(response.status().is_success());
    }
}
