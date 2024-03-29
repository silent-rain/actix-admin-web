use std::env;

mod common;
use common::{schem::create_user_table, user, User};

use database::{Curd, DbRepo, Pagination, Pool};

use sea_orm::{DbErr, EntityTrait, Set};

async fn testcase(db: &Pool) -> Result<(), DbErr> {
    let active_model = user::ActiveModel {
        user_id: Set(1),
        status: Set(1),
        ..Default::default()
    };

    let result = User::insert(active_model)
        .exec(db.wdb())
        .await
        .expect("could not insert baker");

    assert_eq!(result.last_insert_id, 1);

    Ok(())
}

struct UserDao<'a> {
    db: &'a Pool,
}

impl<'a> Curd<user::Entity> for UserDao<'a> {
    type Db = Pool;

    fn db(&self) -> &Self::Db {
        self.db
    }
}

async fn test_all(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
    let dao = UserDao { db };
    let (results, total) = dao.all().await.unwrap();
    println!("test_all ======= {:#?}", results);
    Ok((results, total))
}

async fn test_list(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
    let dao = UserDao { db };
    let page = Pagination::new(0, 10);
    let (results, total) = dao.list(page).await.unwrap();
    println!("test_list ======= {:#?}", results);
    Ok((results, total))
}

// async fn list_pages(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
//     let dao = UserDao { db };
//     let page = Pagination::new(0, 10);
//     let (results, total) = dao.list_pages(page).await.unwrap();
//     println!("list_pages ======= {:#?}", results);
//     Ok((results, total))
// }

async fn test_info(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let result = dao.info(1).await.unwrap().unwrap();
    println!("test_info ======= {:#?}", result);
    Ok(result)
}

async fn test_insert(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::ActiveModel {
        user_id: Set(3),
        status: Set(1),
        ..Default::default()
    };
    let result = dao.insert(data).await.unwrap();
    println!("test_insert ======= {:#?}", result);
    Ok(result)
}

async fn test_insert2(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::Model {
        user_id: 2,
        status: 1,
        ..Default::default()
    };
    let result = dao
        ._insert2::<user::Model, user::ActiveModel>(data)
        .await
        .unwrap();
    println!("test_insert2 ======= {:#?}", result);
    Ok(result)
}

async fn test_update(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::ActiveModel {
        id: Set(2),
        user_id: Set(222),
        status: Set(0),
    };
    let result = dao.update(data).await.unwrap();
    println!("test_update ======= {:#?}", result);
    Ok(result)
}

async fn test_update2(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::Model {
        id: 1,
        user_id: 111,
        status: 0,
    };
    let result = dao
        ._update2::<user::Model, user::ActiveModel>(data)
        .await
        .unwrap();
    println!("test_update2 ======= {:#?}", result);
    Ok(result)
}

async fn test_delete(db: &Pool) -> Result<u64, DbErr> {
    let dao = UserDao { db };

    let result = dao.delete(0).await.unwrap();
    println!("test_delete ======= {:#?}", result);
    Ok(result)
}

async fn test_batch_delete(db: &Pool) -> Result<u64, DbErr> {
    let dao = UserDao { db };

    let result = dao.batch_delete(vec![1, 2]).await.unwrap();
    println!("test_batch_delete ======= {:#?}", result);
    Ok(result)
}

// cargo test --package database --test sqlite_test -- --nocapture
#[tokio::test]
async fn main() -> Result<(), DbErr> {
    env::set_var("RUST_BACKTRACE", "1");

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    // Connecting SQLite
    let db_url = "sqlite::memory:".to_string();
    let db = Pool::connect(db_url).await.expect("db init failed");
    let pool = Pool::form_connect(db.clone(), db);

    // Setup database schema
    create_user_table(&pool).await?;

    // Performing tests
    testcase(&pool).await?;
    test_info(&pool).await?;
    println!("============");

    test_insert(&pool).await?;
    test_insert2(&pool).await?;

    test_list(&pool).await?;
    println!("============");

    test_update(&pool).await?;
    test_update2(&pool).await?;

    test_list(&pool).await?;
    println!("============");

    test_delete(&pool).await?;

    println!("============");

    test_all(&pool).await?;

    test_batch_delete(&pool).await?;

    println!("============");
    test_list(&pool).await?;

    Ok(())
}
