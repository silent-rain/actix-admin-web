//! 用户信息表
//! User Entity: [`entity::prelude::UserProfile`]

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
                    .table(UserProfile::Table)
                    .comment("用户信息表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfile::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::RealName)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Gender)
                            .tiny_integer()
                            .null()
                            .default(1)
                            .comment("性别;1:男,2:女,3:保密"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Password)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Status)
                            .tiny_integer()
                            .not_null()
                            .default(1)
                            .comment("状态,0:停用,1:正常"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Age)
                            .integer()
                            .null()
                            .comment("年龄"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::DateBirth)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Avatar)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("头像URL"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Address)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户的居住或邮寄地址"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Intro)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户个人介绍"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("用户描述"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::Preferences)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("偏好设置"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::DepartmentId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属部门ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::PositionId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属岗位ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::RankId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属职级ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserProfile::UpdatedAt)
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
            .drop_table(Table::drop().table(UserProfile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserProfile {
    #[sea_orm(iden = "t_user_profile")]
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
    Address,
    Intro,
    Desc,
    Preferences,
    DepartmentId,
    PositionId,
    RankId,
    CreatedAt,
    UpdatedAt,
}
