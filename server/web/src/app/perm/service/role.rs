//! 角色管理
use crate::app::perm::{
    dao::role::RoleDao,
    dto::role::{AddRoleReq, RoleListReq, UserRoleListReq},
};

use code::Error;
use entity::perm_role;

use nject::injectable;
use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
#[injectable]
pub struct RoleService<'a> {
    role_dao: RoleDao<'a>,
}

impl<'a> RoleService<'a> {
    /// 获取所有列表数据
    pub async fn all(&self) -> Result<(Vec<perm_role::Model>, u64), Error> {
        let (results, total) = self.role_dao.all().await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok((results, total))
    }

    /// 获取列表数据
    pub async fn list(&self, req: RoleListReq) -> Result<(Vec<perm_role::Model>, u64), Error> {
        let (results, total) = self.role_dao.list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<perm_role::Model>, Error> {
        let result = self.role_dao.info(id).await.map_err(|err| {
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
    pub async fn add(&self, data: AddRoleReq) -> Result<perm_role::Model, Error> {
        let result = self.role_dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.role_dao.delete(id).await.map_err(|err| {
            error!("删除数据失败, error: {err:#?}");
            Error::DBDeleteError
        })?;
        Ok(result)
    }
}

impl<'a> RoleService<'a> {
    /// 通过用户ID获取角色列表
    pub async fn role_list(
        &self,
        req: UserRoleListReq,
    ) -> Result<(Vec<perm_role::Model>, u64), Error> {
        let (results, total) = self.role_dao.role_list(req.user_id).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok((results, total))
    }
}
