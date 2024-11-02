//! OpenApi接口管理
use std::sync::Arc;

use crate::dto::openapi::{GetOpenapiListReq, RoleOpenapiPermission};

use database::{Pagination, PoolTrait};
use entity::{perm_openapi, perm_openapi_role_rel, prelude::PermOpenapi};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct OpenapiDao {
    db: Arc<dyn PoolTrait>,
}

impl OpenapiDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<perm_openapi::Model>, u64), DbErr> {
        let results = PermOpenapi::find()
            .order_by_asc(perm_openapi::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetOpenapiListReq,
    ) -> Result<(Vec<perm_openapi::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermOpenapi::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_openapi::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_openapi::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(perm_openapi::Column::Name.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(perm_openapi::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取父ID下的所有子列表
    pub async fn children(&self, pid: i32) -> Result<Vec<perm_openapi::Model>, DbErr> {
        PermOpenapi::find()
            .filter(perm_openapi::Column::Pid.eq(pid))
            .all(self.db.db())
            .await
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_openapi::Model>, DbErr> {
        PermOpenapi::find_by_id(id).one(self.db.db()).await
    }

    /// 通过资源路径和请求类型获取详情信息
    pub async fn path_info(
        &self,
        path: String,
        method: String,
    ) -> Result<Option<perm_openapi::Model>, DbErr> {
        PermOpenapi::find()
            .filter(perm_openapi::Column::Path.eq(path))
            .filter(perm_openapi::Column::Method.eq(method))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_openapi::ActiveModel,
    ) -> Result<perm_openapi::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: perm_openapi::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermOpenapi::update_many()
            .set(active_model)
            .filter(perm_openapi::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = perm_openapi::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermOpenapi::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}

impl OpenapiDao {
    /// 角色接口关系权限
    pub async fn role_openapi_permissions(&self) -> Result<Vec<RoleOpenapiPermission>, DbErr> {
        let results = PermOpenapi::find()
            .select_only()
            .columns([perm_openapi::Column::Path, perm_openapi::Column::Method])
            .columns([perm_openapi_role_rel::Column::RoleId])
            .join_rev(
                JoinType::InnerJoin,
                perm_openapi_role_rel::Entity::belongs_to(perm_openapi::Entity)
                    .from(perm_openapi_role_rel::Column::OpenapiId)
                    .to(perm_openapi::Column::Id)
                    .into(),
            )
            .filter(perm_openapi::Column::Status.eq(perm_openapi::enums::Status::Enabled as i8))
            .into_model::<RoleOpenapiPermission>()
            .all(self.db.db())
            .await?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use entity::perm_openapi_role_rel;
    use sea_orm::{DatabaseBackend, JoinType};

    #[tokio::test]
    async fn test_sys_user_permission() {
        let results = PermOpenapi::find()
            .join_rev(
                JoinType::InnerJoin,
                perm_openapi_role_rel::Entity::belongs_to(perm_openapi::Entity)
                    .from(perm_openapi_role_rel::Column::OpenapiId)
                    .to(perm_openapi::Column::Id)
                    .into(),
            )
            .filter(perm_openapi_role_rel::Column::RoleId.is_in([1, 2, 3]))
            .build(DatabaseBackend::MySql)
            .to_string();

        println!("{results}");
        assert!(!results.is_empty());
    }
}
