//! 角色管理
use crate::perm::{
    dao::role::RoleDao,
    dto::role::{AddRoleReq, GetRoleListReq},
};

use code::Error;
use entity::perm_role;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务
#[injectable]
pub struct RoleService<'a> {
    role_dao: RoleDao<'a>,
}

impl<'a> RoleService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: GetRoleListReq) -> Result<(Vec<perm_role::Model>, u64), Error> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.role_dao.all().await.map_err(|err| {
                error!("查询角色列表失败, err: {:#?}", err);
                Error::DbQueryError
            });
        }

        let (results, total) = self.role_dao.list(req).await.map_err(|err| {
            error!("查询角色列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_role::Model, Error> {
        let result = self
            .role_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询角色信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("角色不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddRoleReq) -> Result<perm_role::Model, Error> {
        let model = perm_role::ActiveModel {
            name: Set(data.name),
            note: Set(data.note),
            status: Set(1_i8),
            sort: Set(1_i32),
            ..Default::default()
        };
        let result = self.role_dao.add(model).await.map_err(|err| {
            error!("添加角色信息失败, err: {:#?}", err);
            Error::DbAddError
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.role_dao.delete(id).await.map_err(|err| {
            error!("删除角色信息失败, err: {:#?}", err);
            Error::DbDeleteError
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_if_option() {
        let some_option = Some(true);
        if let Some(true) = some_option {
            println!("The option is true!");
        } else {
            println!("The option is not true!");
        }
    }
}
