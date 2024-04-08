//! 创建用户表
//! User Entity: [`entity::prelude::PermUser`]
use entity::{perm_user::Column, prelude::PermUser};

use sea_orm_migration::{
    async_trait,
    sea_orm::DeriveMigrationName,
    sea_query::{ColumnDef, Table},
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
                    .table(PermUser)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(Column::RealName)
                            .string()
                            .string_len(32)
                            .null()
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(Column::Gender)
                            .tiny_integer()
                            .null()
                            .comment("性别;1:男,2:女,3:保密"),
                    )
                    .col(ColumnDef::new(Column::Age).integer().null().comment("年龄"))
                    .col(
                        ColumnDef::new(Column::Birthday)
                            .string()
                            .string_len(20)
                            .null()
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(Column::Avatar)
                            .string()
                            .string_len(200)
                            .null()
                            .comment("头像URL"),
                    )
                    .col(
                        ColumnDef::new(Column::Phone)
                            .string()
                            .string_len(20)
                            .null()
                            .comment("手机号码"),
                    )
                    .col(
                        ColumnDef::new(Column::Email)
                            .string()
                            .string_len(100)
                            .null()
                            .comment("邮箱"),
                    )
                    .col(
                        ColumnDef::new(Column::Intro)
                            .string()
                            .string_len(200)
                            .null()
                            .comment("介绍"),
                    )
                    .col(
                        ColumnDef::new(Column::Note)
                            .string()
                            .string_len(200)
                            .null()
                            .comment("备注"),
                    )
                    .col(
                        ColumnDef::new(Column::Password)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(Column::Status)
                            .tiny_integer()
                            .not_null()
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(Column::DeptId)
                            .integer()
                            .null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(Column::Creator)
                            .integer()
                            .null()
                            .comment("创建者"),
                    )
                    .col(
                        ColumnDef::new(Column::Updater)
                            .integer()
                            .null()
                            .comment("更新者"),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PermUser).to_owned())
            .await
    }
}
