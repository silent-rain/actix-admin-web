//! OpenApi接口角色关系管理
use crate::{
    dao::open_api_role_rel::OpenApiRoleRelDao,
    dto::open_api_role_rel::{BatchAddOpenApiRoleRelReq, GetOpenApiRoleRelListReq},
};

use code::{Error, ErrorMsg};
use entity::perm_open_api_role_rel;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct OpenApiRoleRelService<'a> {
    open_api_role_rel_dao: OpenApiRoleRelDao<'a>,
}

impl<'a> OpenApiRoleRelService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetOpenApiRoleRelListReq,
    ) -> Result<(Vec<perm_open_api_role_rel::Model>, u64), ErrorMsg> {
        let (results, total) = self.open_api_role_rel_dao.list(req).await.map_err(|err| {
            error!("查询OpenApi接口角色关系列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询OpenApi接口角色关系列表失败")
        })?;

        Ok((results, total))
    }

    /// 批量添加数据
    pub async fn batch_add(&self, req: BatchAddOpenApiRoleRelReq) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for role_id in req.role_ids {
            let model = perm_open_api_role_rel::ActiveModel {
                api_id: Set(req.api_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .open_api_role_rel_dao
            .batch_add(models)
            .await
            .map_err(|err| {
                error!("批量添加OpenApi接口角色关系失败, err: {:#?}", err);
                Error::DbBatchAddError
                    .into_msg()
                    .with_msg("批量添加OpenApi接口角色关系失败")
            })?;

        Ok(result)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self
            .open_api_role_rel_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除OpenApi接口角色关系失败, err: {:#?}", err);
                Error::DbBatchDeleteError
                    .into_msg()
                    .with_msg("批量删除OpenApi接口角色关系失败")
            })?;

        Ok(result)
    }
}
