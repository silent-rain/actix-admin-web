//! 用户Token令牌表, 一般openapi服务

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户Token令牌表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_user_token")]
pub struct Model {
    /// 令牌ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 令牌
    pub token: String,
    /// 口令
    pub passphrase: String,
    /// 权限范围:GET,POST,PUT,DELETE
    pub permission: String,
    /// 授权到期时间
    pub expire: i32,
    /// 状态,0:禁用,1:启用
    pub status: i8,
    /// 备注
    pub note: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::perm_user_token_role_rel::Entity")]
    PermUserTokenRoleRel,
}

impl Related<super::perm_user_token_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserTokenRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
