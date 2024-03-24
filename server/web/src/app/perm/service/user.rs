//! 用户管理
use crate::app::perm::{
    dao::{user::UserDao, user_role_rel::UserRoleRelDao},
    dto::user::{AddUserReq, GetUserListReq},
};

use code::Error;
use entity::perm_user;

use nject::injectable;
use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
#[injectable]
pub struct UserService<'a> {
    user_dao: UserDao<'a>,
    user_role_rel_dao: UserRoleRelDao<'a>,
}

impl<'a> UserService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: GetUserListReq) -> Result<(Vec<perm_user::Model>, u64), Error> {
        let (results, total) = self.user_dao.list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<perm_user::Model>, Error> {
        let result = self.user_dao.info(id).await.map_err(|err| {
            if let RecordNotFound(err) = err {
                error!("未查找到数据, error: {err:#?}");
                return Error::DbQueryEmptyError;
            }
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddUserReq) -> Result<perm_user::Model, Error> {
        let result = self.user_dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.user_dao.delete(id).await.map_err(|err| {
            error!("删除数据失败, error: {err:#?}");
            Error::DBDeleteError
        })?;
        Ok(result)
    }
}

impl<'a> UserService<'a> {
    /// 添加用户及对应用户的角色
    pub async fn add_user(&self, data: perm_user::Model) -> Result<perm_user::Model, Error> {
        // 获取待添加的角色列表
        // TODO
        let add_role_ids: Vec<i32> = Vec::new();

        let result = self
            .user_dao
            .add_user(data, add_role_ids)
            .await
            .map_err(|err| {
                error!("添加数据失败, error: {err:#?}");
                Error::DBAddError
            })?;
        Ok(result)
    }

    /// 更新用户及对应用户的角色
    pub async fn update_user(&self, data: perm_user::Model) -> Result<(), Error> {
        // 获取原角色列表
        let (user_role_rels, _) = self.user_role_rel_dao.list(data.id).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;

        // 获取待添加的角色列表
        // TODO
        let add_role_ids: Vec<i32> = Vec::new();
        // 获取待删除的角色列表
        // TODO
        let del_role_ids: Vec<i32> = Vec::new();
        self.user_dao
            .update_user(data, add_role_ids, del_role_ids)
            .await
            .map_err(|err| {
                error!("更新数据失败, error: {err:#?}");
                Error::DBUpdateError
            })?;

        Ok(())
    }
}
