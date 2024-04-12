//! 系统日志
use crate::log::{
    dao::user_login::UserLoginDao,
    dto::user_login::{AddUserLoginInfoReq, GetUserLoginListReq},
};

use code::{Error, ErrorMsg};
use entity::log_user_login;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct UserLoginService<'a> {
    user_login_dao: UserLoginDao<'a>,
}

impl<'a> UserLoginService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserLoginListReq,
    ) -> Result<(Vec<log_user_login::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_login_dao.list(req).await.map_err(|err| {
            error!("查询登陆日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询登陆日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_user_login::Model, ErrorMsg> {
        let result = self
            .user_login_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询登陆日志信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询登陆日志信息失败")
            })?
            .ok_or_else(|| {
                error!("登陆日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("登陆日志不存在")
            })?;

        Ok(result)
    }

    /// 根据用户ID获取详情信息
    pub async fn info_by_user_id(&self, user_id: i32) -> Result<log_user_login::Model, ErrorMsg> {
        let result = self
            .user_login_dao
            .info_by_user_id(user_id)
            .await
            .map_err(|err| {
                error!("查询登陆日志信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询登陆日志信息失败")
            })?
            .ok_or_else(|| {
                error!("登陆日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("登陆日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddUserLoginInfoReq) -> Result<log_user_login::Model, ErrorMsg> {
        let model = log_user_login::ActiveModel {
            user_id: Set(data.user_id),
            username: Set(data.username),
            remote_addr: Set(data.remote_addr),
            user_agent: Set(data.user_agent),
            status: Set(data.status.clone().into()),
            ..Default::default()
        };
        let result = self.user_login_dao.add(model).await.map_err(|err| {
            error!("添加登陆日志信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加登陆日志信息失败")
        })?;

        Ok(result)
    }

    /// 更新登录日志状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.user_login_dao
            .status(id, status)
            .await
            .map_err(|err| {
                error!("更新登录日志状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新登录日志状态失败")
            })?;

        Ok(())
    }
}
