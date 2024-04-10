//! 部门管理
use crate::perm::{
    dao::dept::DeptDao,
    dto::dept::{AddDeptReq, GetDeptListReq, UpdateDeptReq},
    enums::DeptStatus,
};

use code::{Error, ErrorMsg};
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
    pub async fn list(
        &self,
        req: GetDeptListReq,
    ) -> Result<(Vec<perm_dept::Model>, u64), ErrorMsg> {
        // TODO 返回树结构or新接口
        // 获取所有数据
        if let Some(true) = req.all {
            return self.dept_dao.all().await.map_err(|err| {
                error!("查询部门列表失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询部门列表失败")
            });
        }

        let (results, total) = self.dept_dao.list(req).await.map_err(|err| {
            error!("查询部门列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询部门列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_dept::Model, ErrorMsg> {
        let result = self
            .dept_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询部门信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询部门信息失败")
            })?
            .ok_or_else(|| {
                error!("部门不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("部门不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, user_id: i32, req: AddDeptReq) -> Result<perm_dept::Model, ErrorMsg> {
        // 查询部门是否存在
        let dept = self
            .dept_dao
            .info_by_name(req.name.clone())
            .await
            .map_err(|err| {
                error!("查询部门信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询部门信息失败")
            })?;
        if dept.is_some() {
            error!("部门已存在");
            return Err(Error::DbDataExistError.into_msg().with_msg("部门已存在"));
        }

        // TODO pid 待处理
        let model = perm_dept::ActiveModel {
            pid: Set(req.pid),
            // pids: Set(req.pids),
            name: Set(req.name),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(DeptStatus::Enabled as i8),
            creator: Set(Some(user_id)),
            ..Default::default()
        };
        let result = self.dept_dao.add(model).await.map_err(|err| {
            error!("添加部门信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("添加部门信息失败")
        })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, user_id: i32, req: UpdateDeptReq) -> Result<u64, ErrorMsg> {
        // TODO pid 待处理
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
            Error::DbUpdateError.into_msg().with_msg("更新部门失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.dept_dao.status(id, status).await.map_err(|err| {
            error!("更新部门状态失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新部门状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.dept_dao.delete(id).await.map_err(|err| {
            error!("删除部门信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除部门信息失败")
        })?;

        Ok(result)
    }
}