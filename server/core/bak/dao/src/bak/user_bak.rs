//!用户管理
use dto::perm_user::AddUserReq;
use entity::perm_user;
use entity::prelude::PermUser;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DbBackend, DbErr,
    DeleteResult, ExecResult, FromQueryResult, JsonValue, QueryFilter, QueryTrait, Set, Statement,
};
use sea_orm::{DbConn, EntityTrait, PaginatorTrait, QueryOrder};
use serde_json::json;

pub struct Dao;

#[allow(dead_code)]
impl Dao {
    // 获取所有数据
    pub async fn all(db: &DbConn) -> Result<Vec<perm_user::Model>, DbErr> {
        let result = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .all(db)
            .await?;

        Ok(result)
    }

    // 获取数据列表
    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let paginator = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }

    // 获取详情信息
    pub async fn info_by_id(db: &DbConn, id: i32) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find_by_id(id).one(db).await
    }

    // 返回 json 类型数据
    pub async fn info_by_id2(db: &DbConn, id: i32) -> Result<Option<serde_json::Value>, DbErr> {
        let result: Option<serde_json::Value> =
            PermUser::find_by_id(id).into_json().one(db).await?;
        Ok(result)
    }

    // 保存
    pub async fn save(
        db: &DbConn,
        data: perm_user::Model,
    ) -> Result<perm_user::ActiveModel, DbErr> {
        perm_user::ActiveModel {
            nickname: Set(data.nickname),
            gender: Set(data.gender),
            age: Set(data.age),
            phone: Set(data.phone),
            password: Set(data.password),
            status: Set(data.status),
            ..Default::default()
        }
        .save(db)
        .await
    }

    // 保存 - json 数据
    pub async fn save2(
        db: &DbConn,
        data: perm_user::Model,
        _data2: serde_json::Value,
    ) -> Result<perm_user::ActiveModel, DbErr> {
        // A ActiveModel with primary key set
        let mut users = perm_user::ActiveModel {
            id: ActiveValue::Set(data.id),
            nickname: Set(data.nickname),
            gender: Set(data.gender),
            age: Set(data.age),
            phone: Set(data.phone),
            password: Set(data.password),
            status: Set(data.status),
            ..Default::default()
        };

        // Note that this method will not alter the primary key values in ActiveModel
        users.set_from_json(json!({
            "id": 8,
            "nickname": "Apple",
        }))?;

        users.save(db).await
    }

    // 保存 - json 数据
    pub async fn save3(
        db: &DbConn,
        _data: serde_json::Value,
    ) -> Result<perm_user::ActiveModel, DbErr> {
        let users = perm_user::ActiveModel::from_json(json!({
            "id": 8,
            "nickname": "Apple",
        }))?;

        users.save(db).await
    }

    // 插入一个活动模型并返回一个新的 Model .其值是从数据库中检索的，因此将填充任何自动生成的字段。
    pub async fn add(db: &DbConn, data: AddUserReq) -> Result<perm_user::Model, DbErr> {
        let pear = perm_user::ActiveModel {
            nickname: Set(data.nickname),
            gender: Set(data.gender),
            age: Set(Some(data.age)),
            phone: Set(Some(data.phone)),
            password: Set(data.password),
            status: Set(1),
            ..Default::default() // all other attributes are `NotSet`
        };

        pear.insert(db).await
    }

    // 插入活动模型并取回最后一个插入 ID。
    // 其类型与模型的主键类型匹配，因此如果模型具有复合主键，则它可以是元组。
    pub async fn add2(db: &DbConn, data: perm_user::Model) -> Result<i32, DbErr> {
        let pear = perm_user::ActiveModel {
            nickname: Set(data.nickname),
            ..Default::default() // all other attributes are `NotSet`
        };

        let result = PermUser::insert(pear).exec(db).await?;
        Ok(result.last_insert_id)
    }

    // 插入许多活动模型并取回最后一个插入 ID
    pub async fn add_more(db: &DbConn) -> Result<i32, DbErr> {
        let apple = perm_user::ActiveModel {
            nickname: Set("Apple".to_owned()),
            ..Default::default()
        };

        let orange = perm_user::ActiveModel {
            nickname: Set("Orange".to_owned()),
            ..Default::default()
        };

        let result = PermUser::insert_many([apple, orange]).exec(db).await?;
        Ok(result.last_insert_id)
    }

    // 更新
    pub async fn update_by_id(
        db: &DbConn,
        data: perm_user::Model,
    ) -> Result<perm_user::Model, DbErr> {
        perm_user::ActiveModel {
            nickname: Set(data.nickname),
            gender: Set(data.gender),
            age: Set(data.age),
            phone: Set(data.phone),
            password: Set(data.password),
            status: Set(data.status),
            ..Default::default()
        }
        .update(db)
        .await
    }

    // 按主键删除
    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<u64, DbErr> {
        let result = PermUser::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected)
    }

    // 从数据库中查找，然后从数据库中删除相应的行。
    pub async fn delete_by_id2(db: &DbConn, id: i32) -> Result<u64, DbErr> {
        let bean: perm_user::ActiveModel = PermUser::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        let result = bean.delete(db).await?;
        Ok(result.rows_affected)
    }

    // 指定字段删除
    pub async fn delete_by_id3(db: &DbConn, name: String) -> Result<u64, DbErr> {
        let result: DeleteResult = PermUser::delete_many()
            .filter(perm_user::Column::Nickname.contains(&name))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    // 删除所有数据
    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        PermUser::delete_many().exec(db).await
    }
}

// JSON 案例
#[allow(dead_code)]
impl Dao {
    // 获取所有数据 - 返回 json 列表
    pub async fn all2(db: &DbConn) -> Result<Vec<serde_json::Value>, DbErr> {
        let result: Vec<serde_json::Value> = PermUser::find()
            .order_by_asc(perm_user::Column::Nickname)
            .into_json()
            .all(db)
            .await?;

        Ok(result)
    }

    // 获取数据列表 - 返回 json 列表
    pub async fn list2(
        db: &DbConn,
        page: u64,
        page_size: u64,
        name: String,
    ) -> Result<(Vec<serde_json::Value>, u64), DbErr> {
        let paginator = PermUser::find()
            .filter(perm_user::Column::Nickname.contains(&name))
            .order_by_asc(perm_user::Column::Nickname)
            .into_json()
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }
}

// 自定义模型
#[allow(dead_code)]
#[derive(Debug, FromQueryResult)]
pub struct UserUniqueName {
    name: String,
}

// 原始 SQL
#[allow(dead_code)]
impl Dao {
    // 使用适当的语法来绑定参数, 使用适当的语法来绑定参数
    pub async fn get_query_data(
        db: &DbConn,
        name: String,
    ) -> Result<Option<perm_user::Model>, DbErr> {
        let results: Option<perm_user::Model> = PermUser::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT "users"."id", "users"."name" FROM "users" WHERE "id" = $1"#,
                [name.into()],
            ))
            .one(db)
            .await?;
        Ok(results)
    }

    // 选择自定义模型
    pub async fn get_query_list(db: &DbConn) -> Result<Vec<UserUniqueName>, DbErr> {
        let results: Vec<UserUniqueName> =
            UserUniqueName::find_by_statement(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT "users"."name" FROM "users" GROUP BY "users"."name"#,
                [],
            ))
            .all(db)
            .await?;
        Ok(results)
    }

    // 选择不确定的模型
    pub async fn get_query_list2(db: &DbConn) -> Result<Vec<JsonValue>, DbErr> {
        let results: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "users"."name" FROM "users" GROUP BY "users"."name"#,
            [],
        ))
        .all(db)
        .await?;
        Ok(results)
    }

    // 获取数据列表 - 分页
    pub async fn get_query_list3(
        db: &DbConn,
        page: u64,
        page_size: u64,
        name: String,
    ) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let paginator = PermUser::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT "users"."id", "users"."name" FROM "users" WHERE "name" = $1"#,
                [name.into()],
            ))
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, num_pages))
    }

    // 获取原始查询 SQL
    pub async fn get_raw_sql(id: i32) -> String {
        PermUser::find_by_id(id)
            .build(DatabaseBackend::MySql)
            .to_string()
    }
    // 执行 SQL
    pub async fn execute_raw_sql(db: &DbConn) -> Result<u64, DbErr> {
        let exec_res: ExecResult = db
            .execute(Statement::from_string(
                DatabaseBackend::MySql,
                "DROP DATABASE IF EXISTS `sea`;".to_owned(),
            ))
            .await?;
        Ok(exec_res.rows_affected())
    }
}
