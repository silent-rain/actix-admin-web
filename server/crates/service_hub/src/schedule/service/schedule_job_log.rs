//! 定时任务
use crate::schedule::{
    dao::schedule_job_log::ScheduleJobLogDao,
    dto::schedule_job_log::{AddScheduleJobLogReq, GetScheduleJobLogListReq},
};

use code::{Error, ErrorMsg};
use entity::schedule_job_log;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleJobLogService<'a> {
    schedule_job_log_dao: ScheduleJobLogDao<'a>,
}

impl<'a> ScheduleJobLogService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobLogListReq,
    ) -> Result<(Vec<schedule_job_log::Model>, u64), ErrorMsg> {
        let (results, total) = self.schedule_job_log_dao.list(req).await.map_err(|err| {
            error!("查询定时任务日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询定时任务日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_job_log::Model, ErrorMsg> {
        let result = self
            .schedule_job_log_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询定时任务日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询定时任务日志失败")
            })?
            .ok_or_else(|| {
                error!("定时任务日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("定时任务日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        req: AddScheduleJobLogReq,
    ) -> Result<schedule_job_log::Model, ErrorMsg> {
        let data = schedule_job_log::ActiveModel {
            job_id: Set(req.job_id),
            job_name: Set(req.job_name),
            error: Set(req.error),
            cost: Set(req.cost),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let result = self.schedule_job_log_dao.add(data).await.map_err(|err| {
            error!("添加定时任务日志失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("添加定时任务日志失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.schedule_job_log_dao.delete(id).await.map_err(|err| {
            error!("删除定时任务日志失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("删除定时任务日志失败")
        })?;

        Ok(result)
    }
}
