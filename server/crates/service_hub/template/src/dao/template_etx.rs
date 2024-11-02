//! 模板管理
//! 这里收集一些不常用的使用案例

use std::collections::HashMap;

use database::DbRepo;
use entity::{app_template, prelude::AppTemplate};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DbBackend, DbErr,
    DeleteResult, EntityTrait, ExecResult, FromQueryResult, JsonValue, PaginatorTrait, QueryFilter,
    QueryOrder, QueryTrait, Set, Statement,
};
use serde_json::json;

/// 数据访问
#[injectable]
pub struct AppTemplateEtxDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> AppTemplateEtxDao<'a> {
    /// 获取列表数据
    pub async fn list2(
        &self,
        req: AppTemplateListReq,
    ) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = AppTemplate::find()
            .order_by_desc(app_template::Column::Id)
            .paginate(self.db.db(), page.page_size());

        let total = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, total))
    }

    /// 返回 json 类型数据
    pub async fn info(&self, id: i32) -> Result<Option<serde_json::Value>, DbErr> {
        let result: Option<serde_json::Value> = AppTemplate::find_by_id(id)
            .into_json()
            .one(self.db.db())
            .await?;
        Ok(result)
    }

    /// 保存
    /// 这是保存（插入/更新）ActiveModel到数据库的辅助方法。
    /// 保存时ActiveModel，它将根据主键属性执行插入或更新：
    /// - 如果主键是则插入NotSet
    /// - 如果主键是Set或则更新Unchanged
    pub async fn save(
        &self,

        data: app_template::Model,
    ) -> Result<app_template::ActiveModel, DbErr> {
        app_template::ActiveModel {
            user_id: Set(data.user_id),
            status: Set(data.status),
            ..Default::default()
        }
        .save(self.db.db())
        .await
    }

    /// 保存 - json 数据
    pub async fn save2(
        &self,
        data: app_template::Model,
        data2: HashMap<String, String>,
    ) -> Result<app_template::ActiveModel, DbErr> {
        // A ActiveModel with primary key set
        let mut users = app_template::ActiveModel {
            id: ActiveValue::Set(data.id),
            user_id: Set(data.user_id),
            ..Default::default()
        };

        // Note that this method will not alter the primary key values in ActiveModel
        users.set_from_json(json!({
            "status": data2.get("status").unwrap(),
        }))?;

        users.save(self.db.db()).await
    }

    /// 保存 - json 数据
    pub async fn save3(
        &self,
        _data: serde_json::Value,
    ) -> Result<app_template::ActiveModel, DbErr> {
        let users = app_template::ActiveModel::from_json(json!({
            "id": 8,
            "nickname": "Apple",
        }))?;

        users.save(self.db.db()).await
    }

    /// 插入活动模型并取回最后一个插入 ID。
    /// 其类型与模型的主键类型匹配，因此如果模型具有复合主键，则它可以是元组。
    pub async fn add2(&self, data: app_template::Model) -> Result<i32, DbErr> {
        let pear = app_template::ActiveModel {
            user_id: Set(data.user_id),
            ..Default::default() // all other attributes are `NotSet`
        };

        let result = AppTemplate::insert(pear).exec(self.db.db()).await?;
        Ok(result.last_insert_id)
    }

    /// 插入许多活动模型并取回最后一个插入 ID
    pub async fn add_more(&self) -> Result<i32, DbErr> {
        let apple = app_template::ActiveModel {
            user_id: Set("Apple".to_owned()),
            ..Default::default()
        };

        let orange = app_template::ActiveModel {
            user_id: Set("Orange".to_owned()),
            ..Default::default()
        };

        let result = AppTemplate::insert_many([apple, orange])
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 更新
    pub async fn update_by_id(
        &self,
        data: app_template::Model,
    ) -> Result<app_template::Model, DbErr> {
        app_template::ActiveModel {
            user_id: Set(data.user_id),
            status: Set(data.status),
            ..Default::default()
        }
        .update(self.db.db())
        .await
    }

    /// 删除所有数据
    pub async fn delete_all(&self) -> Result<DeleteResult, DbErr> {
        AppTemplate::delete_many().exec(self.db.db()).await
    }
}

/// JSON 案例
impl<'a> AppTemplateEtxDao<'a> {
    /// 获取所有数据 - 返回 json 列表
    pub async fn all2(&self) -> Result<Vec<serde_json::Value>, DbErr> {
        let result: Vec<serde_json::Value> = AppTemplate::find()
            .order_by_asc(app_template::Column::UserId)
            .into_json()
            .all(self.db.db())
            .await?;

        Ok(result)
    }

    /// 获取数据列表 - 返回 json 列表
    pub async fn list2(
        &self,
        page: u64,
        page_size: u64,
        user_id: String,
    ) -> Result<(Vec<serde_json::Value>, u64), DbErr> {
        let paginator = AppTemplate::find()
            .filter(app_template::Column::UserId.contains(user_id))
            .order_by_asc(app_template::Column::UserId)
            .into_json()
            .paginate(self.db.db(), page_size);

        let total = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, total))
    }
}

/// 自定义模型
#[allow(dead_code)]
#[derive(Debug, FromQueryResult)]
pub struct UserUniqueName {
    name: String,
}

/// 原始 SQL
#[allow(dead_code)]
impl<'a> AppTemplateEtxDao<'a> {
    /// 使用适当的语法来绑定参数, 使用适当的语法来绑定参数
    pub async fn get_query_data(&self, name: String) -> Result<Option<app_template::Model>, DbErr> {
        let results: Option<app_template::Model> = AppTemplate::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT "users"."id", "users"."name" FROM "users" WHERE "id" = $1"#,
                [name.into()],
            ))
            .one(self.db.db())
            .await?;
        Ok(results)
    }

    /// 选择自定义模型
    pub async fn get_query_list(&self) -> Result<Vec<UserUniqueName>, DbErr> {
        let results: Vec<UserUniqueName> =
            UserUniqueName::find_by_statement(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT "users"."name" FROM "users" GROUP BY "users"."name"#,
                [],
            ))
            .all(self.db.db())
            .await?;
        Ok(results)
    }

    /// 选择不确定的模型
    pub async fn get_query_list2(&self) -> Result<Vec<JsonValue>, DbErr> {
        let results: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "users"."name" FROM "users" GROUP BY "users"."name"#,
            [],
        ))
        .all(self.db.db())
        .await?;
        Ok(results)
    }

    /// 获取数据列表 - 分页
    pub async fn get_query_list3(
        &self,
        page: u64,
        page_size: u64,
        name: String,
    ) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let paginator = AppTemplate::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT "users"."id", "users"."name" FROM "users" WHERE "name" = $1"#,
                [name.into()],
            ))
            .paginate(self.db.db(), page_size);

        let total = paginator.num_items().await?;

        paginator.fetch_page(page).await.map(|p| (p, total))
    }

    /// 获取原始查询 SQL
    pub async fn get_raw_sql(&self, id: i32) -> String {
        AppTemplate::find_by_id(id)
            .build(DatabaseBackend::MySql)
            .to_string()
    }

    /// 执行 SQL
    pub async fn execute_raw_sql(&self) -> Result<u64, DbErr> {
        let exec_res: ExecResult = self
            .db
            .db()
            .execute(Statement::from_string(
                DatabaseBackend::MySql,
                "DROP DATABASE IF EXISTS `sea`;".to_owned(),
            ))
            .await?;
        Ok(exec_res.rows_affected())
    }
}
