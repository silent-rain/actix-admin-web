//! 部门管理
use crate::perm::{
    dao::dept::DeptDao,
    dto::dept::{AddDeptReq, GetDeptListReq, UpdateDeptReq},
};

use code::Error;
use entity::perm_dept;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct DeptService<'a> {
    dept_dao: DeptDao<'a>,
}

impl<'a> DeptService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: GetDeptListReq) -> Result<(Vec<perm_dept::Model>, u64), Error> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.dept_dao.all().await.map_err(|err| {
                error!("查询部门列表失败, err: {:#?}", err);
                Error::DbQueryError
            });
        }

        let (results, total) = self.dept_dao.list(req).await.map_err(|err| {
            error!("查询部门列表失败, err: {:#?}", err);
            Error::DbQueryError
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_dept::Model, Error> {
        let result = self
            .dept_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询部门信息失败, err: {:#?}", err);
                Error::DbQueryError
            })?
            .ok_or_else(|| {
                error!("部门不存在");
                Error::DbQueryEmptyError
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, user_id: i32, req: AddDeptReq) -> Result<perm_dept::Model, Error> {
        let model = perm_dept::ActiveModel {
            pid: Set(req.pid),
            // pids: Set(req.pids),
            name: Set(req.name),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(req.status),
            creator: Set(Some(user_id)),
            ..Default::default()
        };
        let result = self.dept_dao.add(model).await.map_err(|err| {
            error!("添加部门信息失败, err: {:#?}", err);
            Error::DbAddError
        })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, user_id: i32, req: UpdateDeptReq) -> Result<u64, Error> {
        let model = perm_dept::ActiveModel {
            id: Set(req.id),
            pid: Set(req.pid),
            // pids: Set(req.pids),
            name: Set(req.name),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(req.status),
            updater: Set(Some(user_id)),
            ..Default::default()
        };

        let result = self.dept_dao.update(model).await.map_err(|err| {
            error!("更新部门失败, err: {:#?}", err);
            Error::DbUpdateError
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), Error> {
        self.dept_dao.status(id, status).await.map_err(|err| {
            error!("更新部门状态失败, err: {:#?}", err);
            Error::DbUpdateError
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.dept_dao.delete(id).await.map_err(|err| {
            error!("删除部门信息失败, err: {:#?}", err);
            Error::DbDeleteError
        })?;

        Ok(result)
    }
}
