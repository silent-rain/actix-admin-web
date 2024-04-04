//! 用户管理
use crate::app::perm::{
    dao::{user::UserDao, user_role_rel::UserRoleRelDao},
    dto::user::{AddUserReq, GetUserListReq},
};

use code::Error;
use entity::{perm_role, perm_user};

use nject::injectable;
use sea_orm::Set;
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
            error!("查询用户列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_user::Model, Error> {
        let result = self
            .user_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("用户不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 根据手机号/邮箱获取详情信息
    pub async fn info_by_username(&self, username: String) -> Result<perm_user::Model, Error> {
        let result = self
            .user_dao
            .info_by_username(username)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("用户不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 获取详情数据
    pub async fn info_by_phone(&self, phone: String) -> Result<perm_user::Model, Error> {
        let result = self
            .user_dao
            .info_by_phone(phone)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("用户不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 根据邮箱获取详情信息
    pub async fn info_by_email(&self, email: String) -> Result<perm_user::Model, Error> {
        let result = self
            .user_dao
            .info_by_email(email)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("用户不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddUserReq) -> Result<perm_user::Model, Error> {
        let model = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self.user_dao.add(model).await.map_err(|err| {
            error!("添加用户失败, err: {:#?}", err);
            Error::DbAddError
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.user_dao.delete(id).await.map_err(|err| {
            error!("删除用户失败, err: {:#?}", err);
            Error::DbDeleteError
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

        let data = perm_user::ActiveModel {
            username: Set(data.username),
            gender: Set(data.gender),
            age: Set(data.age),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            phone: Set(data.phone),
            email: Set(data.email),
            password: Set(data.password),
            status: Set(1),
            ..Default::default()
        };

        let result = self
            .user_dao
            .add_user(data, add_role_ids)
            .await
            .map_err(|err| {
                error!("添加用户失败, err: {:#?}", err);
                Error::DbAddError
            })?;
        Ok(result)
    }

    /// 更新用户及对应用户的角色
    pub async fn update_user(&self, data: perm_user::Model) -> Result<(), Error> {
        // 获取原角色列表
        let (user_role_rels, _) = self.user_role_rel_dao.list(data.id).await.map_err(|err| {
            error!("查询用户与角色关系列表失败, err: {:#?}", err);
            Error::DbQueryError
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
                error!("更新用户信息失败, err: {:#?}", err);
                Error::DbUpdateError
            })?;

        Ok(())
    }
}

impl<'a> UserService<'a> {
    /// 通过用户ID获取角色列表
    pub async fn roles(&self, user_id: i32) -> Result<(Vec<perm_role::Model>, u64), Error> {
        let (results, total) = self.user_dao.roles(user_id).await.map_err(|err| {
            error!("查询用户失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }
}
