//! 令牌管理
use crate::{
    dao::token::TokenDao,
    dto::token::{AddTokenReq, GetTokenListReq, UpdateTokenReq},
};

use code::{Error, ErrorMsg};
use entity::perm_token;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use uuid::Uuid;

/// 服务层
#[injectable]
pub struct TokenService {
    token_dao: TokenDao,
}

impl TokenService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetTokenListReq,
    ) -> Result<(Vec<perm_token::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.token_dao.list(req).await.map_err(|err| {
            error!("查询令牌列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询令牌列表失败")
        })?;

        // 屏蔽口令
        for item in results.iter_mut() {
            item.passphrase = "".to_string();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_token::Model, ErrorMsg> {
        let mut result = self
            .token_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询令牌信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询令牌信息失败")
            })?
            .ok_or_else(|| {
                error!("令牌不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("令牌不存在")
            })?;

        // 屏蔽口令
        result.passphrase = "".to_string();
        Ok(result)
    }

    /// 通过Token获取详情信息
    pub async fn info_by_token(
        &self,
        token: String,
        passphrase: String,
    ) -> Result<perm_token::Model, ErrorMsg> {
        let mut result = self
            .token_dao
            .info_by_token(token, passphrase)
            .await
            .map_err(|err| {
                error!("查询令牌信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询令牌信息失败")
            })?
            .ok_or_else(|| {
                error!("令牌不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("令牌不存在")
            })?;

        // 屏蔽口令
        result.passphrase = "".to_string();
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddTokenReq) -> Result<perm_token::Model, ErrorMsg> {
        let token = Uuid::new_v4().to_string();
        let passphrase = Uuid::new_v4().to_string().replace('-', "");
        let model = perm_token::ActiveModel {
            user_id: Set(req.user_id),
            token: Set(token),
            passphrase: Set(passphrase),
            permission: Set(req.permission),
            expire: Set(req.expire),
            desc: Set(req.desc),
            status: Set(perm_token::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result = self
            .token_dao
            .add(model)
            .await
            .map_err(|err: sea_orm::prelude::DbErr| {
                error!("添加令牌信息失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加令牌信息失败")
            })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateTokenReq) -> Result<u64, ErrorMsg> {
        let passphrase = Uuid::new_v4().to_string();
        let model = perm_token::ActiveModel {
            id: Set(id),
            user_id: Set(req.user_id),
            passphrase: Set(passphrase),
            permission: Set(req.permission),
            expire: Set(req.expire),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.token_dao.update(model).await.map_err(|err| {
            error!("更新令牌失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新令牌失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.token_dao.status(id, status).await.map_err(|err| {
            error!("更新令牌状态失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新令牌状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.token_dao.delete(id).await.map_err(|err| {
            error!("删除令牌信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除令牌信息失败")
        })?;

        Ok(result)
    }
}
