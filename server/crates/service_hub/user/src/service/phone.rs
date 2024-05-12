//! 用户手机号管理
use crate::{
    dao::phone::PhoneDao,
    dto::phone::{AddPhoneReq, GetPhoneListReq, UpdatePhoneReq},
};

use code::{Error, ErrorMsg};
use entity::user::user_phone;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct PhoneService<'a> {
    phone_dao: PhoneDao<'a>,
}

impl<'a> PhoneService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetPhoneListReq,
    ) -> Result<(Vec<user_phone::Model>, u64), ErrorMsg> {
        let (results, total) = self.phone_dao.list(req).await.map_err(|err| {
            error!("查询用户手机号列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户手机号列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<user_phone::Model, ErrorMsg> {
        let result = self
            .phone_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户手机号信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户手机号信息失败")
            })?
            .ok_or_else(|| {
                error!("用户手机号不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户手机号不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddPhoneReq) -> Result<user_phone::Model, ErrorMsg> {
        // 检查用户手机号是否已存在
        self.check_phone_exist(req.phone.clone(), None).await?;

        let model = user_phone::ActiveModel {
            user_id: Set(req.user_id),
            phone: Set(req.phone),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.phone_dao.add(model).await.map_err(|err| {
            error!("添加用户手机号信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加用户手机号信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户手机号
    pub async fn update(&self, id: i32, req: UpdatePhoneReq) -> Result<u64, ErrorMsg> {
        // 检查用户手机号是否已存在且不属于当前ID
        self.check_phone_exist(req.phone.clone(), Some(id)).await?;

        let model = user_phone::ActiveModel {
            id: Set(id),
            phone: Set(req.phone),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.phone_dao.update(model).await.map_err(|err| {
            error!("更新用户手机号失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新用户手机号失败")
        })?;

        Ok(result)
    }

    /// 检查手机号是否存在
    async fn check_phone_exist(
        &self,
        phone: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.phone_dao.info_by_phone(phone).await.map_err(|err| {
            error!("查询用户手机号信息失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户手机号信息失败")
        })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("户手机号已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("户手机号已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.phone_dao.delete(id).await.map_err(|err| {
            error!("删除用户手机号失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除用户手机号失败")
        })?;

        Ok(result)
    }
}
