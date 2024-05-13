//! 任务调度状态日志表
//! Entity: [`entity::schedule::ScheduleStatusLog`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Index, Table},
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
                    .table(ScheduleStatusLog::Table)
                    .comment("任务调度状态日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("事件日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Uuid)
                            .string()
                            .string_len(50)
                            .comment("任务调度ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Error)
                            .text()
                            .comment("失败信息"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Cost)
                            .integer()
                            .unsigned()
                            .not_null()
                            .comment("耗时,毫秒"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("任务状态(0:开始,1:完成,2:停止,3:移除)"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::UpdatedAt)
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
            .has_index(ScheduleStatusLog::Table.to_string(), "idx_uuid")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .table(ScheduleStatusLog::Table)
                        .name("idx_uuid")
                        .col(ScheduleStatusLog::Uuid)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ScheduleStatusLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleStatusLog {
    #[sea_orm(iden = "t_schedule_status_log")]
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
