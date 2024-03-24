//! 系统日志
use crate::app::system::{
    dao::user_login::UserLoginDao,
    dto::user_login::{DisableUserLoginReq, UserLoginInfoReq, UserLoginListReq},
};

use code::Error;
use entity::sys_user_login;

use nject::injectable;
use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
#[injectable]
pub struct UserLoginService<'a> {
    user_login_dao: UserLoginDao<'a>,
}

impl<'a> UserLoginService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: UserLoginListReq,
    ) -> Result<(Vec<sys_user_login::Model>, u64), Error> {
        let results = self.user_login_dao.list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        req: UserLoginInfoReq,
    ) -> Result<Option<sys_user_login::Model>, Error> {
        let result = self.user_login_dao.info(req.id).await.map_err(|err| {
            if let RecordNotFound(err) = err {
                error!("未查找到数据, error: {err:#?}");
                return Error::DbQueryEmptyError;
            }
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: sys_user_login::Model) -> Result<sys_user_login::Model, Error> {
        let result = self.user_login_dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 删除数据
    pub async fn disbale_status(&self, req: DisableUserLoginReq) -> Result<(), Error> {
        self.user_login_dao
            .disbale_status(req.id)
            .await
            .map_err(|err| {
                error!("禁用登陆失败, error: {err:#?}");
                Error::DBDeleteError
            })?;
        Ok(())
    }
}
