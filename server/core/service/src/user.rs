//! 用户管理
use code::Error;
use dao::user::Dao;
use database::Pool;
use dto::perm_user::{AddUserReq, UserListReq};
use entity::perm_user::Model;

use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 用户服务
pub struct Service;

impl Service {
    /// 获取列表数据
    pub async fn list(db: &Pool, req: UserListReq) -> Result<(Vec<Model>, u64), Error> {
        let results = Dao::new(db).list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError
        })?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(db: &Pool, id: i32) -> Result<Option<Model>, Error> {
        let result = Dao::new(db).info(id).await.map_err(|err| {
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
    pub async fn add(db: &Pool, data: AddUserReq) -> Result<Model, Error> {
        let result = Dao::new(db).add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }
}
