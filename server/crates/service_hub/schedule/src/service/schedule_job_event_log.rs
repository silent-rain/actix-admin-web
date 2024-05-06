//! 调度任务事件日志管理
use crate::{
    dao::schedule_job_event_log::ScheduleJobEventLogDao,
    dto::schedule_job_event_log::{AddScheduleJobEventLogReq, GetScheduleJobEventLogListReq},
};

use code::{Error, ErrorMsg};
use entity::schedule_job_event_log;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleJobEventLogService<'a> {
    schedule_job_event_log_dao: ScheduleJobEventLogDao<'a>,
}

impl<'a> ScheduleJobEventLogService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobEventLogListReq,
    ) -> Result<(Vec<schedule_job_event_log::Model>, u64), ErrorMsg> {
        let (results, total) = self
            .schedule_job_event_log_dao
            .list(req)
            .await
            .map_err(|err| {
                error!("查询调度任务事件日志列表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询调度任务事件日志列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_job_event_log::Model, ErrorMsg> {
        let result = self
            .schedule_job_event_log_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询调度任务事件日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询调度任务事件日志失败")
            })?
            .ok_or_else(|| {
                error!("调度任务事件日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("调度任务事件日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        req: AddScheduleJobEventLogReq,
    ) -> Result<schedule_job_event_log::Model, ErrorMsg> {
        let data = schedule_job_event_log::ActiveModel {
            job_id: Set(req.job_id),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let result = self
            .schedule_job_event_log_dao
            .add(data)
            .await
            .map_err(|err| {
                error!("添加调度任务事件日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("添加调度任务事件日志失败")
            })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self
            .schedule_job_event_log_dao
            .delete(id)
            .await
            .map_err(|err| {
                error!("删除调度任务事件日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("删除调度任务事件日志失败")
            })?;

        Ok(result)
    }
}
