//! 定时任务日志表
//! User Entity: [`entity::prelude::ScheduleJobLog`]

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
                    .table(ScheduleJobLog::Table)
                    .comment("定时任务日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleJobLog::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobLog::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobLog::Error)
                            .text()
                            .null()
                            .comment("失败信息"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobLog::Cost)
                            .integer()
                            .not_null()
                            .comment("耗时(单位：毫秒)"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobLog::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("任务状态,0:待执行,1:运行中,2:成功,3:失败,4:移除"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJobLog::CreatedAt)
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
            .drop_table(Table::drop().table(ScheduleJobLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleJobLog {
    #[sea_orm(iden = "t_schedule_job_log")]
    Table,
    Id,
    JobId,
    Error,
    Cost,
    Status,
    CreatedAt,
}
