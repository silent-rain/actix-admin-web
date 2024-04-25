//! 系统日志
use crate::{
    dao::user_login::UserLoginDao,
    dto::user_login::{AddUserLoginInfoReq, GetUserLoginListReq},
    enums::UserLoginDisabledStatus,
};

use code::{Error, ErrorMsg};
use entity::log_user_login;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use uap_rust::parser::Parser;

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
        let (mut results, total) = self.user_login_dao.list(req).await.map_err(|err| {
            error!("查询登陆日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询登陆日志列表失败")
        })?;

        // 重置 token 为空
        for item in results.iter_mut() {
            item.token = "".to_string();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_user_login::Model, ErrorMsg> {
        let mut result = self
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

        result.token = "".to_string();
        Ok(result)
    }

    /// 根据Token获取详情信息
    pub async fn info_by_token(&self, token: String) -> Result<log_user_login::Model, ErrorMsg> {
        let mut result = self
            .user_login_dao
            .info_by_token(token)
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
        result.token = "".to_string();
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddUserLoginInfoReq) -> Result<log_user_login::Model, ErrorMsg> {
        let (device, system, browser) = match Parser::new() {
            Ok(p) => {
                let client = p.parse(data.user_agent.clone());
                let device = client.device.family;
                let system = client.os.family;
                let browser = client.user_agent.family;
                (device, system, browser)
            }
            Err(err) => {
                error!("User-Agent解析错误, err: {:#?}", err);
                ("".to_owned(), "".to_owned(), "".to_owned())
            }
        };

        let model = log_user_login::ActiveModel {
            user_id: Set(data.user_id),
            username: Set(data.username),
            token: Set(data.token),
            remote_addr: Set(data.remote_addr),
            user_agent: Set(data.user_agent),
            device: Set(Some(device)),
            system: Set(Some(system)),
            browser: Set(Some(browser)),
            status: Set(data.status as i8),
            disabled: Set(UserLoginDisabledStatus::Enabled as i8),
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

    /// 更新登录日志禁用状态
    pub async fn disabled(&self, id: i32, disabled: i8) -> Result<(), ErrorMsg> {
        self.user_login_dao
            .disabled(id, disabled)
            .await
            .map_err(|err| {
                error!("更新登录日志禁用状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新登录日志禁用状态失败")
            })?;

        Ok(())
    }
}
