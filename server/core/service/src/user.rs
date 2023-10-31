//! 用户管理
use code::Error;
use dao::user::Dao;
use database::DBRepo;
use dto::perm_user::{AddUserReq, UserListReq};
use entity::perm_user::Model;

use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 用户服务
pub struct Service<'a, DB: DBRepo> {
    dao: Dao<'a, DB>,
}

impl<'a, DB: DBRepo> Service<'a, DB> {
    /// 创建对象
    pub fn new(db: &'a DB) -> Self {
        Service { dao: Dao::new(db) }
    }

    /// 获取列表数据
    pub async fn list(&self, req: UserListReq) -> Result<(Vec<Model>, u64), Error> {
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
    pub async fn add(&self, data: AddUserReq) -> Result<Model, Error> {
        let result = self.dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }
}
