//! 用户信息表
//! Entity: [`entity::prelude::UserBase`]

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
                    .table(UserBase::Table)
                    .comment("用户信息表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserBase::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(UserBase::RealName)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Gender)
                            .tiny_integer()
                            .null()
                            .default(1)
                            .comment("性别;1:男,2:女,3:保密"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Password)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Age)
                            .integer()
                            .null()
                            .comment("年龄"),
                    )
                    .col(
                        ColumnDef::new(UserBase::DateBirth)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Avatar)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("头像URL"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Intro)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户个人介绍"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("用户描述"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Address)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户的居住或邮寄地址"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Preferences)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("偏好设置"),
                    )
                    .col(
                        ColumnDef::new(UserBase::DepartmentId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属部门ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::PositionId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属岗位ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::RankId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属职级ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserBase::UpdatedAt)
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
            .drop_table(Table::drop().table(UserBase::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserBase {
    #[sea_orm(iden = "t_user_base")]
    Table,
    Id,
    Username,
    RealName,
    Gender,
    Password,
    Status,
    Age,
    DateBirth,
    Avatar,
    Intro,
    Desc,
    Address,
    Preferences,
    DepartmentId,
    PositionId,
    RankId,
    CreatedAt,
    UpdatedAt,
}
