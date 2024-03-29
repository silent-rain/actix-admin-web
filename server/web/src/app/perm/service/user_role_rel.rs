//! 用户角色关联关系管理
use crate::app::perm::{
    dao::user_role_rel::UserRoleRelDao,
    dto::user_role_rel::{AddUserRoleRelReq, DeleteUserRoleRelReq, GetUserRoleRelListReq},
};

use code::Error;
use entity::perm_user_role_rel;

use nject::injectable;

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
            .map_err(|err| Error::DbQueryError(err.to_string()))?;
        Ok((results, total))
    }

    /// 添加数据
    pub async fn add(&self, req: AddUserRoleRelReq) -> Result<perm_user_role_rel::Model, Error> {
        let result = self
            .user_role_rel_dao
            .add(req.user_id, req.role_id)
            .await
            .map_err(|err| Error::DBAddError(err.to_string()))?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteUserRoleRelReq) -> Result<u64, Error> {
        let result = self
            .user_role_rel_dao
            .delete_by_user_id(req.user_id)
            .await
            .map_err(|err| Error::DBDeleteError(err.to_string()))?;
        Ok(result)
    }
}
