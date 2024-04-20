//! 用户Token令牌管理
use crate::perm::{
    dao::user_token::UserTokenDao,
    dto::user_token::{AddUserTokenReq, GetUserTokenListReq, UpdateUserTokenReq},
    enums::UserTokenStatus,
};

use code::{Error, ErrorMsg};
use entity::perm_user_token;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use uuid::Uuid;

/// 服务层
#[injectable]
pub struct UserTokenService<'a> {
    user_token_dao: UserTokenDao<'a>,
}

impl<'a> UserTokenService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserTokenListReq,
    ) -> Result<(Vec<perm_user_token::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.user_token_dao.list(req).await.map_err(|err| {
            error!("查询用户令牌列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户令牌列表失败")
        })?;

        // 屏蔽口令
        for item in results.iter_mut() {
            item.passphrase = "".to_string();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_user_token::Model, ErrorMsg> {
        let mut result = self
            .user_token_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户令牌信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户令牌信息失败")
            })?
            .ok_or_else(|| {
                error!("用户令牌不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户令牌不存在")
            })?;

        // 屏蔽口令
        result.passphrase = "".to_string();
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddUserTokenReq) -> Result<perm_user_token::Model, ErrorMsg> {
        let token = Uuid::new_v4().to_string();
        let passphrase = Uuid::new_v4().to_string().replace('-', "");
        let model = perm_user_token::ActiveModel {
            user_id: Set(req.user_id),
            token: Set(token),
            passphrase: Set(passphrase),
            permission: Set(req.permission),
            expire: Set(req.expire),
            note: Set(req.note),
            status: Set(UserTokenStatus::Enabled as i8),
            ..Default::default()
        };
        let result =
            self.user_token_dao
                .add(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加用户令牌信息失败, err: {:#?}", err);
                    Error::DbAddError
                        .into_msg()
                        .with_msg("添加用户令牌信息失败")
                })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateUserTokenReq) -> Result<u64, ErrorMsg> {
        let passphrase = Uuid::new_v4().to_string();
        let model = perm_user_token::ActiveModel {
            id: Set(id),
            user_id: Set(req.user_id),
            passphrase: Set(passphrase),
            permission: Set(req.permission),
            expire: Set(req.expire),
            note: Set(req.note),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.user_token_dao.update(model).await.map_err(|err| {
            error!("更新用户令牌失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新用户令牌失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.user_token_dao
            .status(id, status)
            .await
            .map_err(|err| {
                error!("更新用户令牌状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新用户令牌状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.user_token_dao.delete(id).await.map_err(|err| {
            error!("删除用户令牌信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除用户令牌信息失败")
        })?;

        Ok(result)
    }
}
