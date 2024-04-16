//! 定时任务表
//! User Entity: [`entity::prelude::ScheduleJob`]
use entity::{prelude::ScheduleJob, schedule_job::Column};

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
                    .table(ScheduleJob)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("定时任务ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Name)
                            .string()
                            .string_len(200)
                            .not_null()
                            .comment("任务名称"),
                    )
                    .col(
                        ColumnDef::new(Column::JobType)
                            .integer()
                            .not_null()
                            .comment("任务类型,0:定时任务,1:即时任务"),
                    )
                    .col(
                        ColumnDef::new(Column::Expression)
                            .string()
                            .string_len(100)
                            .null()
                            .default("")
                            .comment("cron表达式"),
                    )
                    .col(
                        ColumnDef::new(Column::Interval)
                            .integer()
                            .null()
                            .default(0)
                            .comment("间隔时间,秒"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("配置描述"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("任务状态,0:暂停,1:正常"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ScheduleJob).to_owned())
            .await
    }
}
