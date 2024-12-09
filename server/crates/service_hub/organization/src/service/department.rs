//! 部门管理
use crate::{
    dao::department::DepartmentDao,
    dto::department::{AddDepartmentReq, GetDepartmentListReq, UpdateDepartmentReq},
};

use code::{Error, ErrorMsg};
use entity::organization::department;
use entity::utils::GenericTree;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct DepartmentService {
    department_dao: DepartmentDao,
}

impl DepartmentService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDepartmentListReq,
    ) -> Result<(Vec<department::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.department_dao.all().await.map_err(|err| {
                error!("查询所有部门失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询所有部门失败")
            });
        }

        let (results, total) = self.department_dao.list(req).await.map_err(|err| {
            error!("查询部门列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询部门列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<department::Model>>, ErrorMsg> {
        let (results, _total) = self.department_dao.all().await.map_err(|err| {
            error!("查询部门列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询部门列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);
        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<department::Model, ErrorMsg> {
        let result = self
            .department_dao
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
    pub async fn add(&self, req: AddDepartmentReq) -> Result<department::Model, ErrorMsg> {
        // 检查部门名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        let model = department::ActiveModel {
            pid: Set(req.pid),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let mut department =
            self.department_dao
                .add(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加部门信息失败, err: {:#?}", err);
                    Error::DbAddError.into_msg().with_msg("添加部门信息失败")
                })?;

        // 获取所有部门数据
        let (departments, _) = self.department_dao.all().await.map_err(|err| {
            error!("查询所有部门失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询所有部门失败")
        })?;
        // 获取所有上级ID
        let pids = GenericTree::get_pids(&departments, department.id)
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");

        department.pids = Some(pids);
        // 更新PID
        let model: department::ActiveModel = department.clone().into();
        let _result = self.department_dao.update(model).await.map_err(|err| {
            error!("更新部门Pids失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新部门Pids失败")
        })?;

        Ok(department)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateDepartmentReq) -> Result<u64, ErrorMsg> {
        // 检查部门名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(id)).await?;

        // 获取所有部门数据
        let (departments, _) = self.department_dao.all().await.map_err(|err| {
            error!("查询所有部门失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询所有部门失败")
        })?;
        // 获取所有上级ID
        let pids = GenericTree::get_pids(&departments, id)
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let model = department::ActiveModel {
            id: Set(id),
            pid: Set(req.pid),
            pids: Set(Some(pids)),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.department_dao.update(model).await.map_err(|err| {
            error!("更新部门失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新部门失败")
        })?;

        Ok(result)
    }

    /// 检查部门名称是否存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .department_dao
            .info_by_name(name)
            .await
            .map_err(|err| {
                error!("查询部门信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询部门信息失败")
            })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("部门名称已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("部门名称已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.department_dao
            .status(id, status)
            .await
            .map_err(|err| {
                error!("更新部门状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_msg().with_msg("更新部门状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let children = self.department_dao.children(id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("获取所有子列表失败")
        })?;
        if !children.is_empty() {
            error!("请先删除子列表, children count: {:#?}", children.len());
            return Err(Error::DbDataExistChildrenError
                .into_msg()
                .with_msg("请先删除子列表"));
        }

        let result = self.department_dao.delete(id).await.map_err(|err| {
            error!("删除部门信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除部门信息失败")
        })?;

        Ok(result)
    }
}
