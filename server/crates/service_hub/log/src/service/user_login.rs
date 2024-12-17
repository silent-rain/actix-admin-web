//! 登陆日志管理
use crate::{
    dao::user_login::UserLoginDao,
    dto::user_login::{AddUserLoginInfoReq, GetUserLoginListReq, UpdateUserLoginInfoReq},
};

use code::{Error, ErrorMsg};
use entity::user::user_login_log;
use utils::browser::parse_user_agent_async;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct UserLoginService {
    user_login_dao: UserLoginDao,
}

impl UserLoginService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserLoginListReq,
    ) -> Result<(Vec<user_login_log::Model>, u64), ErrorMsg> {
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
    pub async fn info(&self, id: i32) -> Result<user_login_log::Model, ErrorMsg> {
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
    pub async fn info_by_token(&self, token: String) -> Result<user_login_log::Model, ErrorMsg> {
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
    pub async fn add(&self, req: AddUserLoginInfoReq) -> Result<user_login_log::Model, ErrorMsg> {
        let (device, system, browser) = parse_user_agent_async(req.user_agent.clone())
            .await
            .map_err(|err| {
                error!("User-Agent解析错误, err: {:#?}", err);
                Error::UserAgentParserError(err)
            })?;

        let model = user_login_log::ActiveModel {
            user_id: Set(req.user_id),
            username: Set(req.username),
            token: Set(req.token),
            remote_addr: Set(req.remote_addr),
            user_agent: Set(req.user_agent),
            device: Set(Some(device)),
            system: Set(Some(system)),
            browser: Set(Some(browser)),
            desc: Set(req.desc),
            status: Set(req.status as i8),
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

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateUserLoginInfoReq) -> Result<u64, ErrorMsg> {
        let model = user_login_log::ActiveModel {
            id: Set(id),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.user_login_dao.update(model).await.map_err(|err| {
            error!("更新登陆日志信息失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新登陆日志信息失败")
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
