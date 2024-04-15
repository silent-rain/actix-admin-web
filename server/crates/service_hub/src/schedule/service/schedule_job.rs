//! 定时任务管理
use crate::schedule::{
    dao::schedule_job::ScheduleJobDao,
    dto::schedule_job::{AddcheduleJobReq, GetScheduleJobReq, UpdatecheduleJobReq},
};

use code::{Error, ErrorMsg};
use entity::schedule_job;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleJobService<'a> {
    role_dao: ScheduleJobDao<'a>,
}

impl<'a> ScheduleJobService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobReq,
    ) -> Result<(Vec<schedule_job::Model>, u64), ErrorMsg> {
        let (results, total) = self.role_dao.list(req).await.map_err(|err| {
            error!("查询定时任务列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询定时任务列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_job::Model, ErrorMsg> {
        let result = self
            .role_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询定时任务信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询定时任务信息失败")
            })?
            .ok_or_else(|| {
                error!("定时任务不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("定时任务不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddcheduleJobReq) -> Result<schedule_job::Model, ErrorMsg> {
        // 查询定时任务是否存在
        let role = self
            .role_dao
            .info_by_name(req.name.clone())
            .await
            .map_err(|err| {
                error!("查询定时任务信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询定时任务信息失败")
            })?;
        if role.is_some() {
            error!("定时任务已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("定时任务已存在"));
        }

        let model = schedule_job::ActiveModel {
            name: Set(req.name),
            job_type: Set(req.job_type),
            expression: Set(req.expression),
            interval: Set(req.interval),
            note: Set(req.note),
            status: Set(req.status),
            ..Default::default()
        };
        let result = self.role_dao.add(model).await.map_err(|err| {
            error!("添加定时任务信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加定时任务信息失败")
        })?;

        Ok(result)
    }

    /// 更新定时任务
    pub async fn update(&self, id: i32, req: UpdatecheduleJobReq) -> Result<u64, ErrorMsg> {
        let model = schedule_job::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            job_type: Set(req.job_type),
            expression: Set(req.expression),
            interval: Set(req.interval),
            note: Set(req.note),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.role_dao.update(model).await.map_err(|err| {
            error!("更新定时任务失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新定时任务失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.role_dao.status(id, status).await.map_err(|err| {
            if err == RecordNotUpdated {
                error!("更新定时任务状态失败, 该定时任务不存在");
                return Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新定时任务状态失败, 该定时任务不存在");
            }
            error!("更新定时任务状态失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新定时任务状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.role_dao.delete(id).await.map_err(|err| {
            error!("删除定时任务信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除定时任务信息失败")
        })?;

        Ok(result)
    }
}
