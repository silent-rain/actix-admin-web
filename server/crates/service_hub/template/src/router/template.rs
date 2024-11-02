//! 模板管理

use crate::controller::template::AppTemplateController;

use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

/// 路由器
pub struct AppTemplateRouter;

impl AppTemplateRouter {
    /// 注册路由
    pub fn admin_register() -> Scope {
        scope("/app-templates")
            .route("/all", get().to(AppTemplateController::all))
            .route("", get().to(AppTemplateController::list))
            .route("/{id}", get().to(AppTemplateController::info))
            .route("", post().to(AppTemplateController::add))
            .route("/{id}", put().to(AppTemplateController::update))
            .route("/{id}/status", put().to(AppTemplateController::status))
            .route("/batch", delete().to(AppTemplateController::batch_delete))
            .route("/{id}", delete().to(AppTemplateController::delete))
    }
}

#[cfg(test)]
mod tests {
    use migration::m20230210_145453_create_app_template::Migration;
    use mock_request::Error;
    use mock_request::MockRequest;

    use super::*;

    #[tokio::test]
    async fn test_router_all() -> Result<(), Error> {
        let request = MockRequest::new(AppTemplateRouter::admin_register)
            .await
            .migrations(vec![&Migration])
            .await?
            .enabled_log(true);

        let response = request.assert_get("/app-templates/all", ()).await?;
        println!("response: {:#?}", response);

        // 判断业务状态码
        assert!(response.status() == code::Error::OK.code());

        Ok(())
    }
}
