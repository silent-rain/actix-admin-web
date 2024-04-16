//! 定时任务日志表
//! User Entity: [`entity::prelude::ScheduleJobLog`]
use entity::{prelude::ScheduleJobLog, schedule_job_log::Column};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
    DbErr, MigrationTrait, SchemaManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ScheduleJobLog)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("日志ID"),
                    )
                    .col(
                        ColumnDef::new(Column::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(Column::JobName)
                            .string()
                            .string_len(200)
                            .not_null()
                            .comment("任务名称"),
                    )
                    .col(
                        ColumnDef::new(Column::Error)
                            .text()
                            .null()
                            .comment("失败信息"),
                    )
                    .col(
                        ColumnDef::new(Column::Cost)
                            .decimal()
                            .decimal_len(10, 2)
                            .not_null()
                            .comment("耗时(单位：毫秒)"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("任务状态,0:失败,1:成功"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
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
            .drop_table(Table::drop().table(ScheduleJobLog).to_owned())
            .await
    }
}
