//! 用户邮箱管理
use crate::{
    dao::email::EmailDao,
    dto::email::{AddEmailReq, GetEmailListReq, UpdateEmailReq},
};

use code::{Error, ErrorMsg};
use entity::user_email;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct EmailService<'a> {
    email_dao: EmailDao<'a>,
}

impl<'a> EmailService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetEmailListReq,
    ) -> Result<(Vec<user_email::Model>, u64), ErrorMsg> {
        let (results, total) = self.email_dao.list(req).await.map_err(|err| {
            error!("查询用户邮箱列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户邮箱列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<user_email::Model, ErrorMsg> {
        let result = self
            .email_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户邮箱信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户邮箱信息失败")
            })?
            .ok_or_else(|| {
                error!("用户邮箱不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户邮箱不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddEmailReq) -> Result<user_email::Model, ErrorMsg> {
        // 查询用户邮箱是否已存在
        let email = self
            .email_dao
            .info_by_email(req.email.clone())
            .await
            .map_err(|err| {
                error!("查询用户邮箱信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户邮箱信息失败")
            })?;
        if email.is_some() {
            error!("用户邮箱已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("用户邮箱已存在"));
        }

        let model = user_email::ActiveModel {
            user_id: Set(req.user_id),
            email: Set(req.email),
            note: Set(req.note),
            ..Default::default()
        };
        let result = self.email_dao.add(model).await.map_err(|err| {
            error!("添加用户邮箱信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加用户邮箱信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户邮箱
    pub async fn update(&self, id: i32, req: UpdateEmailReq) -> Result<u64, ErrorMsg> {
        // 查询用户邮箱是否已存在
        let email = self
            .email_dao
            .info_by_email(req.email.clone())
            .await
            .map_err(|err| {
                error!("查询用户邮箱信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户邮箱信息失败")
            })?;
        if email.is_some() {
            error!("用户邮箱已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("用户邮箱已存在"));
        }

        let model = user_email::ActiveModel {
            id: Set(id),
            email: Set(req.email),
            note: Set(req.note),
            ..Default::default()
        };

        let result = self.email_dao.update(model).await.map_err(|err| {
            error!("更新用户邮箱失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新用户邮箱失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.email_dao.delete(id).await.map_err(|err| {
            error!("删除用户邮箱失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除用户邮箱失败")
        })?;

        Ok(result)
    }
}
