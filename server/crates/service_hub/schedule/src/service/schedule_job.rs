//! 任务调度作业管理
use crate::{
    dao::schedule_job::ScheduleJobDao,
    dto::schedule_job::{AddcheduleJobReq, GetScheduleJobReq, UpdatecheduleJobReq},
    ScheduleStatusLogDao,
};

use code::{Error, ErrorMsg};
use entity::schedule::schedule_job;
use scheduler::JobScheduler;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;
use uuid::Uuid;

/// 服务层
#[injectable]
pub struct ScheduleJobService {
    schedule_job_dao: ScheduleJobDao,
    schedule_status_log_dao: ScheduleStatusLogDao,
}

impl ScheduleJobService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobReq,
    ) -> Result<(Vec<schedule_job::Model>, u64), ErrorMsg> {
        let (results, total) = self.schedule_job_dao.list(req).await.map_err(|err| {
            error!("查询调度任务列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询调度任务列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_job::Model, ErrorMsg> {
        let result = self
            .schedule_job_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询调度任务作业失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询调度任务作业失败")
            })?
            .ok_or_else(|| {
                error!("调度任务不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("调度任务不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddcheduleJobReq) -> Result<schedule_job::Model, ErrorMsg> {
        // 检查任务名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        // TODO 添加注册任务

        let model = schedule_job::ActiveModel {
            name: Set(req.name),
            source: Set(req.source as i8),
            job_type: Set(req.job_type as i8),
            sys_code: Set(req.sys_code),
            expression: Set(req.expression),
            interval: Set(req.interval),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.schedule_job_dao.add(model).await.map_err(|err| {
            error!("添加调度任务作业失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加调度任务作业失败")
        })?;

        Ok(result)
    }

    /// 更新调度任务
    pub async fn update(&self, id: i32, req: UpdatecheduleJobReq) -> Result<u64, ErrorMsg> {
        // 检查任务名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(id)).await?;

        let model = schedule_job::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            expression: Set(req.expression),
            interval: Set(req.interval),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.schedule_job_dao.update(model).await.map_err(|err| {
            error!("更新调度任务失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新调度任务失败")
        })?;

        Ok(result)
    }

    /// 检查任务名称是否存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .schedule_job_dao
            .info_by_name(name)
            .await
            .map_err(|err| {
                error!("查询任务名称失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询任务名称失败")
            })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("任务名称已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("任务名称已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.schedule_job_dao
            .status(id, status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新调度任务状态失败, 该调度任务不存在");
                    return Error::DbUpdateError
                        .into_msg()
                        .with_msg("更新调度任务状态失败, 该调度任务不存在");
                }
                error!("更新调度任务状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新调度任务状态失败")
            })?;

        Ok(())
    }

    /// 调度任务下线
    async fn schedule_offline(&self, uuid: String) -> Result<(), ErrorMsg> {
        let job_id =
            Uuid::parse_str(&uuid).map_err(|err| Error::UuidParseError(err.to_string()))?;
        JobScheduler::new()
            .await
            .map_err(|err| {
                error!("获取任务实例失败, err: {:#?}", err);
                Error::ScheduleInstance
                    .into_msg()
                    .with_msg("获取任务实例失败")
            })?
            .remove(&job_id)
            .await
            .map_err(|err| {
                error!("调度任务移除解析失败, err: {:#?}", err);
                Error::ScheduleRemoveError(err.to_string())
                    .into_msg()
                    .with_msg("调度任务移除解析失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let job = self.info(id).await?;
        if job.source == schedule_job::enums::Source::System as i8 {
            error!("系统任务不允许删除");
            return Err(Error::DbDeleteError
                .into_msg()
                .with_msg("系统任务不允许删除"));
        }

        // 调度任务下线
        let status_model = self
            .schedule_status_log_dao
            .last_by_job_id(id)
            .await
            .map_err(|err| {
                error!("查询最新的调度任务作业失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询最新的调度任务作业失败")
            })?
            .ok_or_else(|| {
                error!("调度任务不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("调度任务不存在")
            })?;
        self.schedule_offline(status_model.uuid).await?;

        let result = self.schedule_job_dao.delete(id).await.map_err(|err| {
            error!("删除调度任务作业失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除调度任务作业失败")
        })?;

        Ok(result)
    }
}
