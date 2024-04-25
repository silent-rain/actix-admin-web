//! 系统日志
use crate::{dao::system::SystemDao, dto::system::GetSystemListReq};

use code::{Error, ErrorMsg};
use entity::log_system;

use nject::injectable;
use tracing::error;

/// 服务层
#[injectable]
pub struct SystemService<'a> {
    system_dao: SystemDao<'a>,
}

impl<'a> SystemService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetSystemListReq,
    ) -> Result<(Vec<log_system::Model>, u64), ErrorMsg> {
        let (results, total) = self.system_dao.list(req).await.map_err(|err| {
            error!("查询系统日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询系统日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_system::Model, ErrorMsg> {
        let result = self
            .system_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询系统日志失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询系统日志失败")
            })?
            .ok_or_else(|| {
                error!("系统日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("系统日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: log_system::Model) -> Result<log_system::Model, ErrorMsg> {
        let result = self.system_dao.add(data).await.map_err(|err| {
            error!("添加系统日志失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("添加系统日志失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.system_dao.delete(id).await.map_err(|err| {
            error!("删除系统日志失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("删除系统日志失败")
        })?;

        Ok(result)
    }
}
