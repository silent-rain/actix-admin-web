//! 调度任务表
//! Entity: [`entity::prelude::ScheduleJob`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DatabaseBackend, DeriveIden, DeriveMigrationName,
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
                    .table(ScheduleJob::Table)
                    .comment("调度任务")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleJob::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Name)
                            .string()
                            .string_len(200)
                            .unique_key()
                            .not_null()
                            .comment("任务名称"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Source)
                            .integer()
                            .not_null()
                            .comment("任务来源,0:用户定义,1:系统内部"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::JobType)
                            .integer()
                            .not_null()
                            .comment("任务类型,0:定时任务,1:即时任务"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::SysCode)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("系统任务编码"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Expression)
                            .string()
                            .string_len(100)
                            .null()
                            .default("")
                            .comment("cron表达式"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Interval)
                            .integer()
                            .null()
                            .default(0)
                            .comment("间隔时间,秒"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("配置描述"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("任务状态,0:下线,1:上线"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(ScheduleJob::UpdatedAt)
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ScheduleJob::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleJob {
    #[sea_orm(iden = "t_schedule_job")]
    Table,
    Id,
    Name,
    Source,
    JobType,
    SysCode,
    Expression,
    Interval,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
