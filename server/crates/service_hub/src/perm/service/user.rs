//! 用户管理
use crate::perm::{
    dao::{user::UserDao, user_role_rel::UserRoleRelDao},
    dto::user::{AddUserReq, GetUserListReq, ProfileRsp, UpdateUserReq},
};

use code::{Error, ErrorMsg};
use entity::{perm_role, perm_user, perm_user_role_rel};

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::crypto::sha2_256;

/// 服务层
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

    /// 获取用户个人信息
    pub async fn profile(&self, id: i32) -> Result<ProfileRsp, Error> {
        let user = self
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

        let result = ProfileRsp {
            id,
            username: user.username,
            gender: user.gender,
            age: user.age,
            birthday: user.birthday,
            avatar: user.avatar,
        };
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
    pub async fn add(&self, data: AddUserReq) -> Result<perm_user::Model, ErrorMsg> {
        // 检测是否已注册用户
        if let Some(phone) = data.phone.clone() {
            let user = self.user_dao.info_by_phone(phone).await.map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
            if user.is_some() {
                {
                    error!("该手机号码已注册");
                    return Err(code::Error::DbDataExistError
                        .into_msg()
                        .with_msg("该手机号码已注册"));
                };
            }
        }

        // 检测是否已注册邮件
        if let Some(email) = data.email.clone() {
            let user = self.user_dao.info_by_email(email).await.map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
            if user.is_some() {
                {
                    error!("该邮箱已注册");
                    return Err(code::Error::DbDataExistError
                        .into_msg()
                        .with_msg("该邮箱已注册"));
                };
            }
        }

        // 密码加密
        let password = sha2_256(&data.password);

        let model = perm_user::ActiveModel {
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            phone: Set(data.phone),
            email: Set(data.email),
            password: Set(password),
            status: Set(1),
            ..Default::default()
        };

        let result = self
            .user_dao
            .add_user(model, data.role_ids)
            .await
            .map_err(|err| {
                error!("添加用户失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加用户失败")
            })?;
        Ok(result)
    }

    /// 更新用户及对应用户的角色
    pub async fn update(&self, data: UpdateUserReq) -> Result<(), Error> {
        // 获取原角色列表
        let (user_role_rels, _) = self
            .user_role_rel_dao
            .list_by_user_id(data.id)
            .await
            .map_err(|err| {
                error!("查询用户与角色关系列表失败, err: {:#?}", err);
                Error::DbQueryError
            })?;

        // 获取角色ID的差异列表
        let (add_role_ids, del_role_ids) = self.diff_role_ids(data.role_ids, user_role_rels);

        let model = perm_user::ActiveModel {
            id: Set(data.id),
            username: Set(Some(data.username)),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            birthday: Set(data.birthday),
            avatar: Set(data.avatar),
            phone: Set(data.phone),
            email: Set(data.email),
            password: Set(data.password),
            intro: Set(data.intro),
            note: Set(data.note),
            status: Set(data.status),
            ..Default::default()
        };
        self.user_dao
            .update_user(model, add_role_ids, del_role_ids)
            .await
            .map_err(|err| {
                error!("更新用户信息失败, err: {:#?}", err);
                Error::DbUpdateError
            })?;

        Ok(())
    }

    /// 获取角色ID的差异列表
    fn diff_role_ids(
        &self,
        role_ids: Vec<i32>,
        user_role_rels: Vec<perm_user_role_rel::Model>,
    ) -> (Vec<i32>, Vec<i32>) {
        let raw_role_ids: Vec<i32> = user_role_rels.iter().map(|v| v.role_id).collect();
        // 待新增的ID
        let mut add_role_ids: Vec<i32> = Vec::new();
        for role_id in role_ids.clone().into_iter() {
            if !raw_role_ids.contains(&role_id) {
                add_role_ids.push(role_id);
            }
        }

        // 待删除的ID
        let mut del_role_ids: Vec<i32> = Vec::new();
        for raw_role_id in raw_role_ids.into_iter() {
            if !role_ids.contains(&raw_role_id) {
                del_role_ids.push(raw_role_id);
            }
        }

        (add_role_ids, del_role_ids)
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
