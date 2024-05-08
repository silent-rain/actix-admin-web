//! 任务调度事件日志表
//! Entity: [`entity::schedule::ScheduleEventLog`]

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
                    .table(ScheduleEventLog::Table)
                    .comment("任务调度事件日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleEventLog::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("事件日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleEventLog::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleEventLog::Uuid)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .comment("任务调度ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleEventLog::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("任务状态(0:开始,1:完成,2:停止,3:移除)"),
                    )
                    .col(
                        ColumnDef::new(ScheduleEventLog::CreatedAt)
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
            .drop_table(Table::drop().table(ScheduleEventLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleEventLog {
    #[sea_orm(iden = "t_schedule_event_log")]
    Table,
    Id,
    JobId,
    Uuid,
    Status,
    CreatedAt,
}
