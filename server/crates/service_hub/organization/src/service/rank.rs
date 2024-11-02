//! 职级管理
use crate::{
    dao::rank::RankDao,
    dto::rank::{AddRankReq, GetRankListReq, UpdateRankReq},
};

use code::{Error, ErrorMsg};
use entity::organization::rank;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct RankService {
    rank_dao: RankDao,
}

impl RankService {
    /// 获取列表数据
    pub async fn list(&self, req: GetRankListReq) -> Result<(Vec<rank::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.rank_dao.all().await.map_err(|err| {
                error!("查询所有职级失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询所有职级失败")
            });
        }

        let (results, total) = self.rank_dao.list(req).await.map_err(|err| {
            error!("查询职级列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询职级列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<rank::Model, ErrorMsg> {
        let result = self
            .rank_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询职级信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询职级信息失败")
            })?
            .ok_or_else(|| {
                error!("职级不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("职级不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddRankReq) -> Result<rank::Model, ErrorMsg> {
        // 检查职级名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;
        // 检查职级等级是否已存在
        self.check_level_exist(req.level, None).await?;

        let model = rank::ActiveModel {
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            level: Set(req.level),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let rank = self
            .rank_dao
            .add(model)
            .await
            .map_err(|err: sea_orm::prelude::DbErr| {
                error!("添加职级信息失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加职级信息失败")
            })?;

        Ok(rank)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateRankReq) -> Result<u64, ErrorMsg> {
        // 检查职级名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(id)).await?;
        // 检查职级等级是否已存在且不属于当前ID
        self.check_level_exist(req.level, Some(id)).await?;

        let model = rank::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            level: Set(req.level),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.rank_dao.update(model).await.map_err(|err| {
            error!("更新职级失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新职级失败")
        })?;

        Ok(result)
    }

    /// 检查职级名称是否存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.rank_dao.info_by_name(name).await.map_err(|err| {
            error!("查询职级名称失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询职级名称失败")
        })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("职级名称已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("职级名称已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 检查职级等级是否存在
    async fn check_level_exist(&self, level: u16, current_id: Option<i32>) -> Result<(), ErrorMsg> {
        let result = self.rank_dao.info_by_level(level).await.map_err(|err| {
            error!("查询职级等级失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询职级等级失败")
        })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("职级等级已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("职级等级已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.rank_dao.status(id, status).await.map_err(|err| {
            error!("更新职级状态失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新职级状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.rank_dao.delete(id).await.map_err(|err| {
            error!("删除职级信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除职级信息失败")
        })?;

        Ok(result)
    }
}
