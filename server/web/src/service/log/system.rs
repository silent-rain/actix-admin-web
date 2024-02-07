//! 系统日志
use crate::{dao::log::system::LogSystemDao, dto::log::log_system::LogSystemListReq};

use code::Error;
use database::DBRepo;
use entity::log::system::Model;

use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
pub struct LogSystemService<'a, DB: DBRepo> {
    dao: LogSystemDao<'a, DB>,
}

impl<'a, DB: DBRepo> LogSystemService<'a, DB> {
    /// 创建对象
    pub fn new(db: &'a DB) -> Self {
        LogSystemService { dao: LogSystemDao::new(db) }
    }

    /// 获取列表数据
    pub async fn list(&self, req: LogSystemListReq) -> Result<(Vec<Model>, u64), Error> {
        let results = self.dao.list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError
        })?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<Model>, Error> {
        let result = self.dao.info(id).await.map_err(|err| {
            if let RecordNotFound(err) = err {
                error!("未查找到数据, error: {err:#?}");
                return Error::DbQueryEmptyError;
            }
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryEmptyError
        })?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: Model) -> Result<Model, Error> {
        let result = self.dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.dao.delete(id).await.map_err(|err| {
            error!("删除数据失败, error: {err:#?}");
            Error::DBDeleteError
        })?;
        Ok(result)
    }
}
