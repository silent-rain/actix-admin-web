//! 系统日志
use crate::log::{dao::system::SystemDao, dto::system::GetSystemListReq};

use code::Error;
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
    ) -> Result<(Vec<log_system::Model>, u64), Error> {
        let (results, total) = self.system_dao.list(req).await.map_err(|err| {
            error!("查询系统日志列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_system::Model, Error> {
        let result = self
            .system_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询系统日志失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("系统日志不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: log_system::Model) -> Result<log_system::Model, Error> {
        let result = self.system_dao.add(data).await.map_err(|err| {
            error!("添加系统日志失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.system_dao.delete(id).await.map_err(|err| {
            error!("删除系统日志失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok(result)
    }
}
