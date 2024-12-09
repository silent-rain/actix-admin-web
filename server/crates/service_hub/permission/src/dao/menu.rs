//! 菜单管理
use std::sync::Arc;

use crate::dto::menu::GetMenuListReq;

use database::{Pagination, PoolTrait};
use entity::{permission::menu, permission::Menu};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct MenuDao {
    db: Arc<dyn PoolTrait>,
}

impl MenuDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<menu::Model>, u64), DbErr> {
        let results = Menu::find()
            .order_by_asc(menu::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(&self, req: GetMenuListReq) -> Result<(Vec<menu::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = Menu::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(menu::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(menu::Column::CreatedAt.lt(v))
            })
            .apply_if(req.title, |query, v| {
                query.filter(menu::Column::Title.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(menu::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取父ID下的所有子列表
    pub async fn children(&self, pid: i32) -> Result<Vec<menu::Model>, DbErr> {
        Menu::find()
            .filter(menu::Column::Pid.eq(pid))
            .all(self.db.db())
            .await
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<menu::Model>, DbErr> {
        Menu::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn add(&self, active_model: menu::ActiveModel) -> Result<menu::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: menu::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = Menu::update_many()
            .set(active_model)
            .filter(menu::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = menu::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = Menu::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
