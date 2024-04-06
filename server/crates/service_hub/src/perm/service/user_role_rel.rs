//! 用户角色关联关系管理
use crate::perm::{
    dao::user_role_rel::UserRoleRelDao,
    dto::user_role_rel::{BatchAddUserRoleRelReq, GetUserRoleRelListReq},
};

use code::Error;
use entity::perm_user_role_rel;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct UserRoleRelService<'a> {
    user_role_rel_dao: UserRoleRelDao<'a>,
}

impl<'a> UserRoleRelService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserRoleRelListReq,
    ) -> Result<(Vec<perm_user_role_rel::Model>, u64), Error> {
        let (results, total) = self.user_role_rel_dao.list(req).await.map_err(|err| {
            error!("查询用户与角色关联关系列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 批量添加数据
    pub async fn batch_add(&self, req: BatchAddUserRoleRelReq) -> Result<i32, Error> {
        let mut models = Vec::new();
        for role_id in req.role_ids {
            let model = perm_user_role_rel::ActiveModel {
                user_id: Set(req.user_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .user_role_rel_dao
            .batch_add(models)
            .await
            .map_err(|err| {
                error!("批量添加用户与角色关联关系列表失败, err: {:#?}", err);
                Error::DbBatchAddError
            })?;

        Ok(result)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, Error> {
        let result = self
            .user_role_rel_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除用户与角色关联关系列表失败, err: {:#?}", err);
                Error::DbBatchDeleteError
            })?;

        Ok(result)
    }
}
