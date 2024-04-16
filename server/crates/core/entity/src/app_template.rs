//! 应用模板, 用于制作自定义服务模板

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 应用模板
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_app_template")]
pub struct Model {
    /// 模板ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 状态,0:停用,1:正常
    pub status: i8,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::perm_user::Entity",
        from = "Column::UserId",
        to = "super::perm_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PermUser,
}

impl ActiveModelBehavior for ActiveModel {}
