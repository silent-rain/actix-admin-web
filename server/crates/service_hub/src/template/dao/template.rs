//! 模板管理

use std::str::FromStr;

use crate::template::dto::template::AppTemplateListReq;

use database::{DbRepo, Pagination};
use entity::{app_template, prelude::AppTemplate};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

#[injectable]
pub struct AppTemplateDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> AppTemplateDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let results = AppTemplate::find()
            .order_by_asc(app_template::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    pub async fn list(
        &self,
        req: AppTemplateListReq,
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

    /// 获取列表数据
    pub async fn list2(
        &self,
        req: AppTemplateListReq,
    ) -> Result<(Vec<app_template::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let paginator = AppTemplate::find()
            .order_by_desc(app_template::Column::Id)
            .paginate(self.db.rdb(), page.page_size());

        let total = paginator.num_items().await?;

        paginator
            .fetch_page(page.page())
            .await
            .map(|results| (results, total))
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
    use sea_orm::DbBackend;

    use super::*;

    #[test]
    fn test_all() {
        let result = AppTemplate::find()
            .order_by_asc(app_template::Column::Id)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `app_template`.`id`, `app_template`.`user_id`, `app_template`.`status`, `app_template`.`created_at`, `app_template`.`updated_at` FROM `app_template` ORDER BY `app_template`.`id` ASC"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_list() {}

    #[test]
    fn test_info() {
        let result = AppTemplate::find_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `app_template`.`id`, `app_template`.`user_id`, `app_template`.`status`, `app_template`.`created_at`, `app_template`.`updated_at` FROM `app_template` WHERE `app_template`.`id` = 1"#;

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

        let sql = r#"INSERT INTO `app_template` (`id`, `user_id`, `status`) VALUES (1, 11, 1)"#;

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

        let sql = r#"INSERT INTO `app_template` (`id`, `user_id`, `status`) VALUES (1, 11, 1), (2, 22, 0)"#;

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

        let sql = r#"UPDATE `app_template` SET `id` = 1, `user_id` = 11, `status` = 1 WHERE `app_template`.`id` = 1"#;

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

        let sql = r#"UPDATE `app_template` SET `status` = 0 WHERE `app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_delete() {
        let result = AppTemplate::delete_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `app_template` WHERE `app_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_batch_delete() {
        let ids = vec![1, 2, 3, 4];
        let result = AppTemplate::delete_many()
            .filter(app_template::Column::Id.is_in(ids))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `app_template` WHERE `app_template`.`id` IN (1, 2, 3, 4)"#;

        assert_eq!(result, sql);
    }
}
