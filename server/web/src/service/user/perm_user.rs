//! 用户管理
use crate::dao::user::perm_user::PermUserDao;
use crate::dto::user::perm_user::{AddUserReq, UserListReq};

use code::Error;
use entity::perm_user::Model;

use nject::injectable;
use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
#[injectable]
pub struct PermUserService<'a> {
    dao: PermUserDao<'a>,
}

impl<'a> PermUserService<'a> {
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

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.dao.delete(id).await.map_err(|err| {
            error!("删除数据失败, error: {err:#?}");
            Error::DBDeleteError
        })?;
        Ok(result)
    }
}
