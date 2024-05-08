//! 用户信息管理
use crate::{
    dao::{user_base::UserBaseDao, user_role_rel::UserRoleRelDao},
    dto::user_base::{AddUserBaseReq, GetUserBaserListReq, ProfileRsp, UpdateUserBaseReq},
};

use code::{Error, ErrorMsg};
use entity::{perm_role, user_base, user_role_rel};

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::crypto::sha2_256;

/// 服务层
#[injectable]
pub struct UserBaseService<'a> {
    user_base_dao: UserBaseDao<'a>,
    user_role_rel_dao: UserRoleRelDao<'a>,
}

impl<'a> UserBaseService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserBaserListReq,
    ) -> Result<(Vec<user_base::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_base_dao.list(req).await.map_err(|err| {
            error!("查询用户信息列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户信息列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<user_base::Model, ErrorMsg> {
        let result = self
            .user_base_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("用户信息不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户信息不存在")
            })?;

        Ok(result)
    }

    /// 获取用户信息个人信息
    pub async fn profile(&self, id: i32) -> Result<ProfileRsp, ErrorMsg> {
        let user = self
            .user_base_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("用户信息不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户信息不存在")
            })?;

        let result = ProfileRsp {
            id,
            username: user.username,
            gender: user.gender as i8,
            age: user.age,
            date_birth: user.date_birth,
            avatar: user.avatar,
        };
        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.user_base_dao.status(id, status).await.map_err(|err| {
            error!("更新用户信息状态失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新用户信息状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.user_base_dao.delete(id).await.map_err(|err| {
            error!("删除用户信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除用户信息失败")
        })?;

        Ok(result)
    }
}

impl<'a> UserBaseService<'a> {
    /// 后台添加用户信息及对应用户信息的角色
    pub async fn add(&self, data: AddUserBaseReq) -> Result<user_base::Model, ErrorMsg> {
        // 密码加密
        let password = sha2_256(&data.password);

        let model = user_base::ActiveModel {
            username: Set(data.username),
            real_name: Set(data.real_name),
            gender: Set(data.gender as i8),
            password: Set(password),
            status: Set(data.status as i8),
            age: Set(data.age),
            date_birth: Set(data.date_birth),
            avatar: Set(data.avatar),
            intro: Set(data.intro),
            desc: Set(data.desc),
            address: Set(data.address),
            preferences: Set(data.preferences),
            department_id: Set(data.department_id),
            position_id: Set(data.position_id),
            rank_id: Set(data.rank_id),
            ..Default::default()
        };

        let result = self
            .user_base_dao
            .add_user(model, data.role_ids)
            .await
            .map_err(|err| {
                error!("添加用户信息失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加用户信息失败")
            })?;
        Ok(result)
    }

    /// 后台更新用户信息及对应用户信息的角色
    pub async fn update(&self, id: i32, data: UpdateUserBaseReq) -> Result<(), ErrorMsg> {
        // 获取原角色列表
        let (user_role_rels, _) =
            self.user_role_rel_dao
                .list_by_user_id(id)
                .await
                .map_err(|err| {
                    error!("查询用户信息与角色关系列表失败, err: {:#?}", err);
                    Error::DbQueryError
                        .into_msg()
                        .with_msg("查询用户信息与角色关系列表失败")
                })?;

        // 获角色色ID的差异列表
        let (add_role_ids, del_role_ids) = self.diff_role_ids(data.role_ids, user_role_rels);

        let model = user_base::ActiveModel {
            id: Set(id),
            username: Set(data.username),
            real_name: Set(data.real_name),
            gender: Set(data.gender as i8),
            status: Set(data.status as i8),
            age: Set(data.age),
            date_birth: Set(data.date_birth),
            avatar: Set(data.avatar),
            intro: Set(data.intro),
            desc: Set(data.desc),
            address: Set(data.address),
            preferences: Set(data.preferences),
            department_id: Set(data.department_id),
            position_id: Set(data.position_id),
            rank_id: Set(data.rank_id),
            ..Default::default()
        };
        self.user_base_dao
            .update_user(model, add_role_ids, del_role_ids)
            .await
            .map_err(|err| {
                error!("更新用户信息失败, err: {:#?}", err);
                Error::DbUpdateError.into_msg().with_msg("更新用户信息失败")
            })?;

        Ok(())
    }

    /// 获角色色ID的差异列表
    fn diff_role_ids(
        &self,
        role_ids: Vec<i32>,
        user_role_rels: Vec<user_role_rel::Model>,
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

impl<'a> UserBaseService<'a> {
    /// 通过用户信息ID获角色色列表
    pub async fn roles(&self, user_id: i32) -> Result<(Vec<perm_role::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_base_dao.roles(user_id).await.map_err(|err| {
            error!("查询用户信息失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
        })?;

        Ok((results, total))
    }
}
