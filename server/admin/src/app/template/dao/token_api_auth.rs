//! API 管理
//! 这里演示一个表关联的写法
use database::DbRepo;
use entity::api_role_http_rel;
use entity::app_template;
use entity::prelude::{ApiRoleHttpRel, AppTemplate};
use entity::sys_user_login;

use nject::injectable;
use sea_orm::{DbErr, EntityTrait, JoinType, QuerySelect, RelationTrait};
use sea_query::{Expr, IntoCondition};

#[injectable]
pub struct AppTemplateEtxDao<'a> {
    db: &'a dyn DbRepo,
}

// JOIN 联表

impl<'a> AppTemplateEtxDao<'a> {
    /// join 案例
    pub async fn get_join_list(&self) -> Result<Vec<api_role_http_rel::Model>, DbErr> {
        let results = ApiRoleHttpRel::find()
            // reuse a `Relation` from existing Entity
            .join(
                JoinType::LeftJoin,
                api_role_http_rel::Relation::ApiHttp.def(),
            )
            .join(
                JoinType::LeftJoin,
                api_role_http_rel::Relation::PermRole
                    .def()
                    .rev()
                    .on_condition(|_left, right| {
                        Expr::col((right, api_role_http_rel::Column::RoleId))
                            .gt(10i32)
                            .into_condition()
                    }),
            )
            // join with table alias and custom on condition
            .join(
                JoinType::LeftJoin,
                api_role_http_rel::Relation::ApiHttp
                    .def()
                    .on_condition(|_left, right| {
                        Expr::col((right, api_role_http_rel::Column::ApiId))
                            .like("%lemon%")
                            .into_condition()
                    }),
            )
            .all(self.db.rdb())
            .await?;
        Ok(results)
    }

    /// join 案例
    /// ``` rust
    /// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    /// pub enum Relation {
    ///            #[sea_orm(
    ///               belongs_to = "super::perm_user::Entity",
    ///                from = "Column::UserId",
    ///                to = "super::perm_user::Column::Id",
    ///                on_update = "Cascade",
    ///                on_delete = "Cascade"
    ///           )]
    ///           PermUser,
    ///       }
    /// ```
    pub async fn get_join_list2(&self) -> Result<Vec<app_template::Model>, DbErr> {
        let results = AppTemplate::find()
            // reuse a `Relation` from existing Entity
            .join(JoinType::LeftJoin, app_template::Relation::PermUser.def())
            // construct `RelationDef` on the fly
            .join_rev(
                JoinType::InnerJoin,
                sys_user_login::Entity::belongs_to(app_template::Entity)
                    .from(sys_user_login::Column::UserId)
                    .to(app_template::Column::UserId)
                    .into(),
            )
            .all(self.db.rdb())
            .await?;
        Ok(results)
    }
}