//! 调度任务事件日志表
//! User Entity: [`entity::prelude::ScheduleJobEventLog`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DeriveIden, DeriveMigrationName,
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
                    .table(ScheduleJobEventLog::Table)
                    .comment("调度任务事件日志")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleJobEventLog::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("事件日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobEventLog::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobEventLog::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("任务状态,0:开始,1:完成,2:停止,3:移除"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobEventLog::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ScheduleJobEventLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleJobEventLog {
    #[sea_orm(iden = "t_schedule_job_event_log")]
    Table,
    Id,
    JobId,
    Status,
    CreatedAt,
}
