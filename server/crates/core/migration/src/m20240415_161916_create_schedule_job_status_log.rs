//! 调度任务状态日志表
//! User Entity: [`entity::prelude::ScheduleJobStatus`]
use crate::m20240415_161916_create_schedule_job::ScheduleJob;

use sea_orm::{
    sea_query::{ColumnDef, Expr, ForeignKey, Index, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ScheduleJobStatus::Table)
                    .comment("调度任务状态日志")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleJobStatus::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("事件日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::Uuid)
                            .string()
                            .string_len(50)
                            .default("")
                            .comment("调度任务ID, 每次任务动态变化"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::Error)
                            .text()
                            .comment("失败信息"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::Cost)
                            .integer()
                            .unsigned()
                            .not_null()
                            .comment("耗时,毫秒"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("任务状态,0:开始,1:完成,2:停止,3:移除"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobStatus::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra({
                                match manager.get_database_backend() {
                                    DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
                                    _ => "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                                }
                            })
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(ScheduleJobStatus::Table.to_string(), "uk_job_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("uk_job_id")
                        .table(ScheduleJobStatus::Table)
                        .unique()
                        .col(ScheduleJobStatus::JobId)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                ScheduleJobStatus::Table.to_string(),
                "fk_schedule_job_status_job_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_schedule_job_status_job_id")
                        .from(ScheduleJobStatus::Table, ScheduleJobStatus::JobId)
                        .to(ScheduleJob::Table, ScheduleJob::Id)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ScheduleJobStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleJobStatus {
    #[sea_orm(iden = "t_schedule_job_status_log")]
    Table,
    Id,
    JobId,
    Uuid,
    Error,
    Cost,
    Status,
    CreatedAt,
    UpdatedAt,
}
