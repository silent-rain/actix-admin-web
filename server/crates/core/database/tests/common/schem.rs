use database::{DbRepo, Pool};

use sea_orm::{
    sea_query::{ColumnDef, Table, TableCreateStatement},
    ConnectionTrait, DbBackend, DbErr, EntityTrait, ExecResult, Schema,
};

use super::user;
use super::User;

pub async fn create_user_table(db: &Pool) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(User)
        .col(
            ColumnDef::new(user::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key()
                .comment("ID"),
        )
        .col(
            ColumnDef::new(user::Column::UserId)
                .integer()
                .not_null()
                .comment("用户ID"),
        )
        .col(
            ColumnDef::new(user::Column::Status)
                .tiny_integer()
                .not_null()
                .default(1)
                .comment("登录状态,0:禁用,1:启用"),
        )
        .to_owned();

    create_table(db, &stmt, User).await
}

pub async fn create_table<E>(
    db: &Pool,
    create: &TableCreateStatement,
    entity: E,
) -> Result<ExecResult, DbErr>
where
    E: EntityTrait,
{
    let builder = db.wdb().get_database_backend();
    let schema = Schema::new(builder);
    assert_eq!(
        builder.build(&schema.create_table_from_entity(entity)),
        builder.build(create)
    );

    create_table_without_asserts(db, create).await
}

pub async fn create_table_without_asserts(
    db: &Pool,
    create: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    let builder = db.wdb().get_database_backend();
    if builder != DbBackend::Sqlite {
        let stmt = builder.build(
            Table::drop()
                .table(create.get_table_name().unwrap().clone())
                .if_exists()
                .cascade(),
        );
        db.wdb().execute(stmt).await?;
    }
    db.wdb().execute(builder.build(create)).await
}
