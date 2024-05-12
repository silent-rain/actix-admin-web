//! 任务调度作业管理
use crate::{
    dao::schedule_job::ScheduleJobDao,
    dto::schedule_job::{AddcheduleJobReq, GetScheduleJobReq, UpdatecheduleJobReq},
};

use code::{Error, ErrorMsg};
use entity::schedule::schedule_job;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleJobService<'a> {
    schedule_job_dao: ScheduleJobDao<'a>,
}

impl<'a> ScheduleJobService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleJobReq,
    ) -> Result<(Vec<schedule_job::Model>, u64), ErrorMsg> {
        let (results, total) = self.schedule_job_dao.list(req).await.map_err(|err| {
            error!("查询任务调度列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询任务调度列表失败")
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
                error!("查询任务调度作业失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询任务调度作业失败")
            })?
            .ok_or_else(|| {
                error!("任务调度不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("任务调度不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddcheduleJobReq) -> Result<schedule_job::Model, ErrorMsg> {
        // 检查任务名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        let model = schedule_job::ActiveModel {
            name: Set(req.name),
            source: Set(req.source as i8),
            job_type: Set(req.job_type as i8),
            sys_code: Set(req.sys_code),
            expression: Set(req.expression),
            interval: Set(req.interval),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };
        let result = self.schedule_job_dao.add(model).await.map_err(|err| {
            error!("添加任务调度作业失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加任务调度作业失败")
        })?;

        Ok(result)
    }

    /// 更新任务调度
    pub async fn update(&self, id: i32, req: UpdatecheduleJobReq) -> Result<u64, ErrorMsg> {
        // 检查任务名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(id)).await?;

        let model = schedule_job::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            expression: Set(req.expression),
            interval: Set(req.interval),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.schedule_job_dao.update(model).await.map_err(|err| {
            error!("更新任务调度失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新任务调度失败")
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
                    error!("更新任务调度状态失败, 该任务调度不存在");
                    return Error::DbUpdateError
                        .into_msg()
                        .with_msg("更新任务调度状态失败, 该任务调度不存在");
                }
                error!("更新任务调度状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新任务调度状态失败")
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

        let result = self.schedule_job_dao.delete(id).await.map_err(|err| {
            error!("删除任务调度作业失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除任务调度作业失败")
        })?;

        Ok(result)
    }
}
