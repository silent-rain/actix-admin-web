//! 模板管理
use crate::app::template::{
    dao::template::AppTemplateDao,
    dto::template::{AddAppTemplateStatusReq, AppTemplateListReq},
};

use code::Error;
use entity::app_template;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务
#[injectable]
pub struct AppTemplateService<'a> {
    app_template_dao: AppTemplateDao<'a>,
}

impl<'a> AppTemplateService<'a> {
    /// 获取所有{{InterfaceName}}数据
    pub async fn all(&self) -> Result<(Vec<app_template::Model>, u64), Error> {
        let (results, total) = self.app_template_dao.all().await.map_err(|err| {
            error!("查询{{InterfaceName}}列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取{{InterfaceName}}列表
    pub async fn list(
        &self,
        req: AppTemplateListReq,
    ) -> Result<(Vec<app_template::Model>, u64), Error> {
        let (results, total) = self.app_template_dao.list(req).await.map_err(|err| {
            error!("查询{{InterfaceName}}列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取{{InterfaceName}}详情
    pub async fn info(&self, id: i32) -> Result<app_template::Model, Error> {
        let result = self
            .app_template_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询{{InterfaceName}}信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("{{InterfaceName}}不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加{{InterfaceName}}
    pub async fn add(&self, data: AddAppTemplateStatusReq) -> Result<app_template::Model, Error> {
        let data = app_template::ActiveModel {
            user_id: Set(data.user_id),
            status: Set(data.status),
            ..Default::default()
        };

        let result = self.app_template_dao.add(data).await.map_err(|err| {
            error!("添加{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbAddError
        })?;

        Ok(result)
    }

    /// 更新{{InterfaceName}}
    pub async fn update(&self, id: i32, status: i8) -> Result<u64, Error> {
        let data = app_template::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };

        let result = self.app_template_dao.update(data).await.map_err(|err| {
            error!("更新{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbUpdateError
        })?;

        Ok(result)
    }

    /// 更新{{InterfaceName}}状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), Error> {
        self.app_template_dao
            .status(id, status)
            .await
            .map_err(|err| {
                error!("更新{{InterfaceName}}状态失败, err: {:#?}", err);
                Error::DbUpdateError
            })?;

        Ok(())
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.app_template_dao.delete(id).await.map_err(|err| {
            error!("删除{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbDeleteError
        })?;

        Ok(result)
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, Error> {
        let result = self
            .app_template_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除{{InterfaceName}}失败, err: {:#?}", err);
                Error::DbBatchDeleteError
            })?;

        Ok(result)
    }
}
