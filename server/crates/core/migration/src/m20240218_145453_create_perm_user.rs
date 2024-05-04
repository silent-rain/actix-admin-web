//! 创建用户表
//! User Entity: [`entity::prelude::PermUser`]

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
                    .table(PermUser::Table)
                    .comment("用户表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermUser::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(PermUser::RealName)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Gender)
                            .tiny_integer()
                            .null()
                            .default(1)
                            .comment("性别;1:男,2:女,3:保密"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Age)
                            .integer()
                            .null()
                            .comment("年龄"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Birthday)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Avatar)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("头像URL"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Intro)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("介绍"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Note)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Password)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(PermUser::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(PermUser::DeptId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(PermUser::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(PermUser::UpdatedAt)
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
            .drop_table(Table::drop().table(PermUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PermUser {
    #[sea_orm(iden = "t_perm_user")]
    Table,
    Id,
    Username,
    RealName,
    Gender,
    Age,
    Birthday,
    Avatar,
    Intro,
    Note,
    Password,
    Status,
    DeptId,
    CreatedAt,
    UpdatedAt,
}
