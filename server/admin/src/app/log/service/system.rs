//! 系统日志
use crate::app::log::{dao::system::LogSystemDao, dto::system::LogSystemListReq};

use code::Error;
use entity::log::system;

use nject::injectable;

/// 服务
#[injectable]
pub struct LogSystemService<'a> {
    dao: LogSystemDao<'a>,
}

impl<'a> LogSystemService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: LogSystemListReq) -> Result<(Vec<system::Model>, u64), Error> {
        let results = self
            .dao
            .list(req)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<system::Model, Error> {
        let result = self
            .dao
            .info(id)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?
            .ok_or(Error::DbQueryEmptyError)?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: system::Model) -> Result<system::Model, Error> {
        let result = self
            .dao
            .add(data)
            .await
            .map_err(|err| Error::DBAddError(err.to_string()))?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self
            .dao
            .delete(id)
            .await
            .map_err(|err| Error::DBDeleteError(err.to_string()))?;
        Ok(result)
    }
}
