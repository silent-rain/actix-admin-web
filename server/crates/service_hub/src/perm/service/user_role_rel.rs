//! 用户角色关联关系管理
use crate::perm::{
    dao::user_role_rel::UserRoleRelDao,
    dto::user_role_rel::{AddUserRoleRelReq, GetUserRoleRelListReq},
};

use code::Error;
use entity::perm_user_role_rel;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务
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
        let (results, total) = self
            .user_role_rel_dao
            .list(req.user_id)
            .await
            .map_err(|err| {
                error!("查询用户与角色关联关系列表失败, err: {:#?}", err);
                Error::DbQueryError
            })?;

        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(&self, req: AddUserRoleRelReq) -> Result<perm_user_role_rel::Model, Error> {
        let model = perm_user_role_rel::ActiveModel {
            user_id: Set(req.user_id),
            role_id: Set(req.role_id),
            ..Default::default()
        };

        let result = self.user_role_rel_dao.add(model).await.map_err(|err| {
            error!("添加用户与角色关联关系列表失败, err: {:#?}", err);
            Error::DbAddError
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, user_id: i32) -> Result<u64, Error> {
        let result = self
            .user_role_rel_dao
            .delete_by_user_id(user_id)
            .await
            .map_err(|err| {
                error!("删除用户与角色关联关系列表失败, err: {:#?}", err);
                Error::DbDeleteError
            })?;

        Ok(result)
    }
}
