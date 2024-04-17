//! 角色管理
use crate::schedule::dto::schedule_job::GetScheduleJobReq;

use database::{DbRepo, Pagination};
use entity::{prelude::ScheduleJob, schedule_job};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct ScheduleJobDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> ScheduleJobDao<'a> {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetScheduleJobReq,
    ) -> Result<(Vec<schedule_job::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ScheduleJob::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(schedule_job::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(schedule_job::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(schedule_job::Column::Name.like(format!("{v}%")))
            })
            .apply_if(req.job_type, |query, v| {
                query.filter(schedule_job::Column::JobType.eq(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(schedule_job::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(schedule_job::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<schedule_job::Model>, DbErr> {
        ScheduleJob::find_by_id(id).one(self.db.rdb()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(&self, name: String) -> Result<Option<schedule_job::Model>, DbErr> {
        ScheduleJob::find()
            .filter(schedule_job::Column::Name.eq(name))
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: schedule_job::ActiveModel,
    ) -> Result<schedule_job::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: schedule_job::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleJob::update_many()
            .set(active_model)
            .filter(schedule_job::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = schedule_job::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ScheduleJob::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }
}
