//! Mock 测试

mod common;
use common::User;

use database::mock::Mock;
use sea_orm::{ConnectionTrait, DbBackend, DbErr, EntityTrait, Statement};
use std::env;
use tracing::info;

async fn mock_pool() -> Result<(), DbErr> {
    let pool = Mock::connect().await;

    // 创建表并返回pool
    let sql = r#"CREATE TABLE `user` 
    (
        `id` INT(11) NULL,
        `user_id` INT(10) NOT NULL,
        `status` INT(10) NOT NULL,
        PRIMARY KEY (`id`)
    );"#;
    let result = pool.db().execute_unprepared(sql).await?;
    info!("result1: {:#?}", result);

    // 插入数据
    let sql = r#"INSERT INTO user (id,user_id, status)
     VALUES (1,1,1);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::MySql, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result2: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());
    Ok(())
}

async fn mock_entity() -> Result<(), DbErr> {
    // 创建表并返回pool
    let pool = Mock::from_entity(User).await?;

    // 插入数据
    let sql = r#"INSERT INTO user (id,user_id, status)
     VALUES (1,1,1);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::Sqlite, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());
    Ok(())
}

async fn mock_str() -> Result<(), DbErr> {
    // 创建表并返回pool
    let sql = r#"CREATE TABLE `user` 
    (
        `id` INT(11) NULL,
        `user_id` INT(10) NOT NULL,
        `status` INT(10) NOT NULL,
        PRIMARY KEY (`id`)
    );"#;
    let pool = Mock::from_str(sql).await?;

    // 插入数据
    let sql = r#"INSERT INTO user (id,user_id, status)
     VALUES (1,1,1);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::Sqlite, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());

    Ok(())
}

// cargo test --package database --test mock_test -- --nocapture
#[tokio::test]
async fn main() -> Result<(), DbErr> {
    env::set_var("RUST_BACKTRACE", "1");

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    mock_pool().await?;
    mock_entity().await?;
    mock_str().await?;

    Ok(())
}
