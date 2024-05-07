//! OpenApi接口管理
use crate::{
    dao::openapi::OpenapiDao,
    dto::openapi::{AddOpenapiReq, GetOpenapiListReq, UpdateOpenapiReq},
};

use code::{Error, ErrorMsg};
use entity::perm_openapi;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::list_tree::GenericTree;

/// 服务层
#[injectable]
pub struct OpenapiService<'a> {
    openapi_dao: OpenapiDao<'a>,
}

impl<'a> OpenapiService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetOpenapiListReq,
    ) -> Result<(Vec<perm_openapi::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.openapi_dao.all().await.map_err(|err| {
                error!("查询所有OpenApi接口失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询所有OpenApi接口失败")
            });
        }

        let (results, total) = self.openapi_dao.list(req).await.map_err(|err| {
            error!("查询OpenApi接口列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询OpenApi接口列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<perm_openapi::Model>>, ErrorMsg> {
        let (results, _total) = self.openapi_dao.all().await.map_err(|err| {
            error!("查询OpenApi接口列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询OpenApi接口列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_openapi::Model, ErrorMsg> {
        let result = self
            .openapi_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询OpenApi接口信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询OpenApi接口信息失败")
            })?
            .ok_or_else(|| {
                error!("OpenApi接口不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("OpenApi接口不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddOpenapiReq) -> Result<perm_openapi::Model, ErrorMsg> {
        // 查询OpenApi接口是否已存在
        let open_api = self
            .openapi_dao
            .path_info(req.path.clone(), req.method.clone())
            .await
            .map_err(|err| {
                error!("查询OpenApi接口信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询OpenApi接口信息失败")
            })?;
        if open_api.is_some() {
            error!("OpenApi接口已存在, 请不要重复注册");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("OpenApi接口已存在, 请不要重复注册"));
        }

        let model = perm_openapi::ActiveModel {
            pid: Set(req.pid),
            category: Set(req.category as i8),
            name: Set(req.name),
            method: Set(req.method),
            path: Set(req.path),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(perm_openapi::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result =
            self.openapi_dao
                .add(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加OpenApi接口信息失败, err: {:#?}", err);
                    Error::DbAddError
                        .into_msg()
                        .with_msg("添加OpenApi接口信息失败")
                })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateOpenapiReq) -> Result<u64, ErrorMsg> {
        let model = perm_openapi::ActiveModel {
            id: Set(id),
            pid: Set(req.pid),
            category: Set(req.category as i8),
            name: Set(req.name),
            method: Set(req.method),
            path: Set(req.path),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.openapi_dao.update(model).await.map_err(|err| {
            error!("更新OpenApi接口失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新OpenApi接口失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.openapi_dao.status(id, status).await.map_err(|err| {
            error!("更新OpenApi接口状态失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新OpenApi接口状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let children = self.openapi_dao.children(id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("获取所有子列表失败")
        })?;
        if !children.is_empty() {
            error!("请先删除子列表, children count: {:#?}", children.len());
            return Err(Error::DbDataExistChildrenError
                .into_msg()
                .with_msg("请先删除子列表"));
        }

        let result = self.openapi_dao.delete(id).await.map_err(|err| {
            error!("删除OpenApi接口信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除OpenApi接口信息失败")
        })?;

        Ok(result)
    }
}
