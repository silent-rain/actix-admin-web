//! 配置管理
use crate::{
    dao::config::ConfigDao,
    dto::config::{AddConfigReq, GetConfigListReq, UpdateConfigReq},
};

use code::{Error, ErrorMsg};
use entity::sys_config;
use entity::utils::GenericTree;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct ConfigService {
    config_dao: ConfigDao,
}

impl ConfigService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetConfigListReq,
    ) -> Result<(Vec<sys_config::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.config_dao.all().await.map_err(|err| {
                error!("查询配置列表失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询配置列表失败")
            });
        }

        let (results, total) = self.config_dao.list(req).await.map_err(|err| {
            error!("查询配置列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询配置列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<sys_config::Model>>, ErrorMsg> {
        let (results, _total) = self.config_dao.all().await.map_err(|err| {
            error!("查询配置列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询配置列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);

        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<sys_config::Model, ErrorMsg> {
        let result = self
            .config_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询配置信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询配置信息失败")
            })?
            .ok_or_else(|| {
                error!("配置不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("配置不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddConfigReq) -> Result<sys_config::Model, ErrorMsg> {
        // 查询配置编码是否存在
        self.check_code_exist(req.code.clone(), None).await?;

        let model = sys_config::ActiveModel {
            pid: Set(req.pid),
            name: Set(req.name),
            code: Set(req.code),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(sys_config::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result = self.config_dao.add(model).await.map_err(|err| {
            error!("添加配置信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("添加配置信息失败")
        })?;

        Ok(result)
    }

    /// 更新配置
    pub async fn update(&self, id: i32, req: UpdateConfigReq) -> Result<u64, ErrorMsg> {
        // 查询配置编码是否存在且不属于当前ID
        self.check_code_exist(req.code.clone(), Some(id)).await?;

        let model = sys_config::ActiveModel {
            id: Set(id),
            pid: Set(req.pid),
            name: Set(req.name),
            code: Set(req.code),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.config_dao.update(model).await.map_err(|err| {
            error!("更新配置失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新配置失败")
        })?;

        Ok(result)
    }

    /// 检查配置编码是否存在
    async fn check_code_exist(
        &self,
        code: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.config_dao.info_by_code(code).await.map_err(|err| {
            error!("查询配置编码失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询配置编码失败")
        })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("配置编码已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("配置编码已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.config_dao.status(id, status).await.map_err(|err| {
            if err == RecordNotUpdated {
                error!("更新配置状态失败, 该配置不存在");
                return Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新配置状态失败, 该配置不存在");
            }
            error!("更新配置状态失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新配置状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let config_children = self.config_dao.children(id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("获取所有子列表失败")
        })?;
        if !config_children.is_empty() {
            error!(
                "请先删除子列表, children count: {:#?}",
                config_children.len()
            );
            return Err(Error::DbDataExistChildrenError
                .into_msg()
                .with_msg("请先删除子列表"));
        }

        let result = self.config_dao.delete(id).await.map_err(|err| {
            error!("删除配置信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除配置信息失败")
        })?;

        Ok(result)
    }
}
