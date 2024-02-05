//! API 管理
use entity::api_role_http_rel;
use entity::prelude::ApiRoleHttpRel;

use sea_orm::{DbConn, DbErr, EntityTrait, JoinType, QuerySelect, RelationTrait};
use sea_query::{Expr, IntoCondition};

pub struct Dao;

// JOIN 联表
#[allow(dead_code)]
impl Dao {
    // join 案例
    pub async fn get_join_list(db: &DbConn) -> Result<Vec<api_role_http_rel::Model>, DbErr> {
        let results = ApiRoleHttpRel::find()
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
            .all(db)
            .await?;
        Ok(results)
    }
}
