//! 系统日志
use crate::app::system::{
    dao::user_login::UserLoginDao,
    dto::user_login::{UserLoginInfoReq, UserLoginListReq, UserLoginStatusReq},
};

use code::Error;
use entity::sys_user_login;

use nject::injectable;

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
        let results = self
            .user_login_dao
            .list(req)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?;
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, req: UserLoginInfoReq) -> Result<sys_user_login::Model, Error> {
        let result = self
            .user_login_dao
            .info(req.id)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?
            .ok_or(Error::DbQueryEmptyError)?;
        Ok(result)
    }

    /// 根据用户ID获取详情信息
    pub async fn info_by_user_id(&self, user_id: i32) -> Result<sys_user_login::Model, Error> {
        let result = self
            .user_login_dao
            .info_by_user_id(user_id)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?
            .ok_or(Error::DbQueryEmptyError)?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        data: sys_user_login::ActiveModel,
    ) -> Result<sys_user_login::Model, Error> {
        let result = self
            .user_login_dao
            .add(data)
            .await
            .map_err(|err| Error::DBAddError(err.to_string()))?;
        Ok(result)
    }

    /// 更新登录日志状态
    pub async fn status(&self, req: UserLoginStatusReq) -> Result<(), Error> {
        self.user_login_dao
            .status(req.id, req.status)
            .await
            .map_err(|err| Error::DBDeleteError(err.to_string()))?;
        Ok(())
    }
}
