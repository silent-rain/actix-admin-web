//! 角色部门关系管理
use crate::perm::{
    dao::role_dept_rel::RoleDeptRelDao,
    dto::role_dept_rel::{BatchAddRoleDeptRelReq, GetRoleDeptRelListReq},
};

use code::Error;
use entity::perm_role_dept_rel;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct RoleDeptRelService<'a> {
    role_dept_rel_dao: RoleDeptRelDao<'a>,
}

impl<'a> RoleDeptRelService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetRoleDeptRelListReq,
    ) -> Result<(Vec<perm_role_dept_rel::Model>, u64), Error> {
        let (results, total) = self.role_dept_rel_dao.list(req).await.map_err(|err| {
            error!("查询角色与部门关系列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 批量添加数据
    pub async fn batch_add(&self, user_id: i32, req: BatchAddRoleDeptRelReq) -> Result<i32, Error> {
        let mut models = Vec::new();
        for role_id in req.role_ids {
            let model = perm_role_dept_rel::ActiveModel {
                role_id: Set(role_id),
                dept_id: Set(req.dept_id),
                creator: Set(Some(user_id)),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .role_dept_rel_dao
            .batch_add(models)
            .await
            .map_err(|err| {
                error!("批量添加角色与部门关系列表失败, err: {:#?}", err);
                Error::DbBatchAddError
            })?;

        Ok(result)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, Error> {
        let result = self
            .role_dept_rel_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除角色与部门关系列表失败, err: {:#?}", err);
                Error::DbBatchDeleteError
            })?;

        Ok(result)
    }
}
