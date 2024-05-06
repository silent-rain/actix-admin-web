//! 调度任务状态日志管理
use crate::{
    dao::schedule_job_status_log::ScheduleJobStatusLogDao,
    dto::schedule_job_status_log::{
        AddScheduleJobStatusLogReq, GetScheduleJobStatusListLogReq, UpdateScheduleJobStatusLogReq,
    },
};

use code::{Error, ErrorMsg};
use entity::schedule_job_status_log;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleJobStatusLogService<'a> {
    schedule_job_status_log_dao: ScheduleJobStatusLogDao<'a>,
}

impl<'a> ScheduleJobStatusLogService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobStatusListLogReq,
    ) -> Result<(Vec<schedule_job_status_log::Model>, u64), ErrorMsg> {
        let (results, total) = self
            .schedule_job_status_log_dao
            .list(req)
            .await
            .map_err(|err| {
                error!("查询调度任务状态日志列表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询调度任务状态日志列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_job_status_log::Model, ErrorMsg> {
        let result = self
            .schedule_job_status_log_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询调度任务状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询调度任务状态日志失败")
            })?
            .ok_or_else(|| {
                error!("调度任务状态日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("调度任务状态日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        req: AddScheduleJobStatusLogReq,
    ) -> Result<schedule_job_status_log::Model, ErrorMsg> {
        let data = schedule_job_status_log::ActiveModel {
            job_id: Set(req.job_id),
            uuid: Set(req.uuid),
            ..Default::default()
        };
        let result = self
            .schedule_job_status_log_dao
            .add(data)
            .await
            .map_err(|err| {
                error!("添加调度任务状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("添加调度任务状态日志失败")
            })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(
        &self,
        id: i32,
        req: UpdateScheduleJobStatusLogReq,
    ) -> Result<u64, ErrorMsg> {
        let model = schedule_job_status_log::ActiveModel {
            id: Set(id),
            job_id: Set(req.job_id),
            uuid: Set(req.uuid),
            error: Set(req.error),
            cost: Set(req.cost),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self
            .schedule_job_status_log_dao
            .update(model)
            .await
            .map_err(|err| {
                error!("更新调度任务状态日志失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新调度任务状态日志失败")
            })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.schedule_job_status_log_dao
            .status(id, status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新调度任务状态日志失败, 该调度任务状态日志不存在");
                    return Error::DbUpdateError
                        .into_msg()
                        .with_msg("更新调度任务状态日志失败, 该调度任务状态日志不存在");
                }
                error!("更新调度任务状态日志失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新调度任务状态日志失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self
            .schedule_job_status_log_dao
            .delete(id)
            .await
            .map_err(|err| {
                error!("删除调度任务状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("删除调度任务状态日志失败")
            })?;

        Ok(result)
    }
}
