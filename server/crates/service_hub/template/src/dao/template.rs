//! 模板管理

use std::str::FromStr;

use crate::dto::template::GetAppTemplateListReq;

use database::{ArcDbRepo, Pagination};
use entity::{app_template, prelude::AppTemplate};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct AppTemplateDao {
    pub db: ArcDbRepo,
}

impl AppTemplateDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let results = AppTemplate::find()
            .order_by_asc(app_template::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetAppTemplateListReq,
    ) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = AppTemplate::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(app_template::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(app_template::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let order_by_col = match req.order_by {
            Some(v) => app_template::Column::from_str(&v).map_or(app_template::Column::Id, |v| v),
            None => app_template::Column::Id,
        };

        let results = states
            .order_by_desc(order_by_col)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<app_template::Model>, DbErr> {
        AppTemplate::find_by_id(id).one(self.db.rdb()).await
    }

    /// 添加数据
    pub async fn add(
        &self,
        active_model: app_template::ActiveModel,
    ) -> Result<app_template::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 批量添加数据
    pub async fn batch_add(
        &self,
        active_models: Vec<app_template::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = AppTemplate::insert_many(active_models)
            .exec(self.db.wdb())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 更新数据
    pub async fn update(&self, active_model: app_template::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = AppTemplate::update_many()
            .set(active_model)
            .filter(app_template::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = app_template::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.wdb()).await?;
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = AppTemplate::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = AppTemplate::delete_many()
            .filter(app_template::Column::Id.is_in(ids))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use database::mock::Mock;
    use migration::m20230210_145453_create_app_template::Migration;

    use sea_orm::DbBackend;

    #[test]
    fn test_all() {
        let result = AppTemplate::find()
            .order_by_asc(app_template::Column::Id)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_app_template`.`id`, `t_app_template`.`user_id`, `t_app_template`.`status`, `t_app_template`.`created_at`, `t_app_template`.`updated_at` FROM `t_app_template` ORDER BY `t_app_template`.`id` ASC"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_list() {}

    #[test]
    fn test_info() {
        let result = AppTemplate::find_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_app_template`.`id`, `t_app_template`.`user_id`, `t_app_template`.`status`, `t_app_template`.`created_at`, `t_app_template`.`updated_at` FROM `t_app_template` WHERE `t_app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_add() {
        let active_model = app_template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(1),
            ..Default::default()
        };
        let result = AppTemplate::insert(active_model)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"INSERT INTO `t_app_template` (`id`, `user_id`, `status`) VALUES (1, 11, 1)"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_batch_add() {
        let active_model1 = app_template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(1),
            ..Default::default()
        };
        let active_model2 = app_template::ActiveModel {
            id: Set(2),
            user_id: Set(22),
            status: Set(0),
            ..Default::default()
        };
        let models = [active_model1, active_model2];
        let result = AppTemplate::insert_many(models)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"INSERT INTO `t_app_template` (`id`, `user_id`, `status`) VALUES (1, 11, 1), (2, 22, 0)"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_update() {
        let active_model = app_template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(1),
            ..Default::default()
        };
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = AppTemplate::update_many()
            .set(active_model)
            .filter(app_template::Column::Id.eq(id))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"UPDATE `t_app_template` SET `id` = 1, `user_id` = 11, `status` = 1 WHERE `t_app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_status() {
        let active_model = app_template::ActiveModel {
            id: Set(1),
            status: Set(0),
            ..Default::default()
        };
        let result = AppTemplate::update(active_model)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"UPDATE `t_app_template` SET `status` = 0 WHERE `t_app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_delete() {
        let result = AppTemplate::delete_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `t_app_template` WHERE `t_app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_batch_delete() {
        let ids = vec![1, 2, 3, 4];
        let result = AppTemplate::delete_many()
            .filter(app_template::Column::Id.is_in(ids))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `t_app_template` WHERE `t_app_template`.`id` IN (1, 2, 3, 4)"#;

        assert_eq!(result, sql);
    }

    #[tokio::test]
    async fn test_mock_all() -> Result<(), DbErr> {
        let pool = Mock::from_migration(&Migration).await?;

        let dao = AppTemplateDao { db: pool };

        let (results, total) = dao.all().await?;
        assert!(results.is_empty());
        assert!(total == 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_mock_info() -> Result<(), Box<DbErr>> {
        let pool = Mock::from_migration(&Migration).await?;

        let dao = AppTemplateDao { db: pool };

        let result = dao.info(1).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_mock_add() -> Result<(), DbErr> {
        let pool = Mock::from_migration(&Migration).await?;

        let dao = AppTemplateDao { db: pool };

        // 添加模板1
        let active_model = app_template::ActiveModel {
            user_id: Set(1),
            desc: Set(Some("desc".to_string())),
            status: Set(1),
            ..Default::default()
        };
        let result = dao.add(active_model).await?;
        println!("add result1: {result:#?}");
        assert!(result.user_id == 1);

        // 添加模板2
        let active_model = app_template::ActiveModel {
            user_id: Set(2),
            desc: Set(Some("desc2".to_string())),
            status: Set(0),
            ..Default::default()
        };
        let result = dao.add(active_model).await?;
        println!("add result2: {result:#?}");
        assert!(result.user_id == 2);

        // 查询模板1
        let result = dao.info(1).await?;
        println!("info result: {result:#?}");
        assert!(result.is_some());

        // 查询所有的模板
        let (results, total) = dao.all().await?;
        println!("all results: {results:#?}");
        assert!(!results.is_empty());
        assert!(total == 2);

        Ok(())
    }
}
