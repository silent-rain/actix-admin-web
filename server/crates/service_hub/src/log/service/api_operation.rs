//! API操作日志
use crate::log::{
    dao::api_operation::ApiOperationDao,
    dto::api_operation::{AddApiOperationReq, GetApiOperationListReq},
};

use code::{Error, ErrorMsg};
use entity::log_api_operation;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct ApiOperationService<'a> {
    system_dao: ApiOperationDao<'a>,
}

impl<'a> ApiOperationService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetApiOperationListReq,
    ) -> Result<(Vec<log_api_operation::Model>, u64), ErrorMsg> {
        let (results, total) = self.system_dao.list(req).await.map_err(|err| {
            error!("查询操作日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询操作日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_api_operation::Model, ErrorMsg> {
        let result = self
            .system_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询操作日志失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询操作日志失败")
            })?
            .ok_or_else(|| {
                error!("操作日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("操作日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddApiOperationReq) -> Result<log_api_operation::Model, ErrorMsg> {
        let data = log_api_operation::ActiveModel {
            user_id: Set(req.user_id),
            username: Set(req.username),
            request_id: Set(req.request_id),
            status_code: Set(req.status_code),
            method: Set(req.method),
            path: Set(req.path),
            query: Set(req.query),
            body: Set(req.body),
            remote_addr: Set(req.remote_addr),
            user_agent: Set(req.user_agent),
            cost: Set(req.cost),
            http_type: Set(req.http_type),
            note: Set(req.note),
            ..Default::default()
        };
        let result = self.system_dao.add(data).await.map_err(|err| {
            error!("添加操作日志失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("添加操作日志失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.system_dao.delete(id).await.map_err(|err| {
            error!("删除操作日志失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("删除操作日志失败")
        })?;

        Ok(result)
    }
}
