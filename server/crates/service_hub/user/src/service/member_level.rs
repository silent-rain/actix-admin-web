//! 会员等级管理
use crate::{
    dao::member_level::MemberLevelDao,
    dto::member_level::{AddMemberLevelReq, GetMemberLevelListReq, UpdateMemberLevelReq},
};

use code::{Error, ErrorMsg};
use entity::user::member_level;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct MemberLevelService<'a> {
    member_level_dao: MemberLevelDao<'a>,
}

impl<'a> MemberLevelService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetMemberLevelListReq,
    ) -> Result<(Vec<member_level::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.member_level_dao.all().await.map_err(|err| {
                error!("查询所有会员等级失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询所有会员等级失败")
            });
        }

        let (results, total) = self.member_level_dao.list(req).await.map_err(|err| {
            error!("查询会员等级列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询会员等级列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<member_level::Model, ErrorMsg> {
        let result = self
            .member_level_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询会员等级信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询会员等级信息失败")
            })?
            .ok_or_else(|| {
                error!("会员等级不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("会员等级不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddMemberLevelReq) -> Result<member_level::Model, ErrorMsg> {
        // 查询会员等级名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;
        // 检查会员等级是否已存在且不属于当前ID
        self.check_level_exist(req.level, None).await?;

        let model = member_level::ActiveModel {
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            level: Set(req.level),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let member_level =
            self.member_level_dao
                .add(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加会员等级信息失败, err: {:#?}", err);
                    Error::DbAddError
                        .into_msg()
                        .with_msg("添加会员等级信息失败")
                })?;

        Ok(member_level)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateMemberLevelReq) -> Result<u64, ErrorMsg> {
        // 查询会员等级名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(id)).await?;
        // 检查会员等级是否已存在且不属于当前ID
        self.check_level_exist(req.level, Some(id)).await?;

        let model = member_level::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            level: Set(req.level),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.member_level_dao.update(model).await.map_err(|err| {
            error!("更新会员等级失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新会员等级失败")
        })?;

        Ok(result)
    }

    /// 查询会员等级名称是否已存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .member_level_dao
            .info_by_name(name)
            .await
            .map_err(|err| {
                error!("查询会员等级信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询会员等级信息失败")
            })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("会员等级名称已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("会员等级名称已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 检查会员等级是否存在
    async fn check_level_exist(&self, level: u16, current_id: Option<i32>) -> Result<(), ErrorMsg> {
        let result = self
            .member_level_dao
            .info_by_level(level)
            .await
            .map_err(|err| {
                error!("查询会员等级失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询会员等级失败")
            })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("会员等级已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("会员等级已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.member_level_dao
            .status(id, status)
            .await
            .map_err(|err| {
                error!("更新会员等级状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新会员等级状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.member_level_dao.delete(id).await.map_err(|err| {
            error!("删除会员等级信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除会员等级信息失败")
        })?;

        Ok(result)
    }
}
