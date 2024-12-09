//! 令牌表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 令牌表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_token")]
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
    pub expire: DateTimeLocal,
    /// 状态,0:禁用,1:启用
    pub status: i8,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::token_role_rel::Entity")]
    PermTokenRoleRel,
}

impl Related<super::token_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermTokenRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 令牌状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }

    /// 令牌权限范围
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Permission {
        /// 读取数据
        #[serde(rename = "GET")]
        GET,
        /// 提交数据
        #[serde(rename = "POST")]
        POST,
        /// 更新数据
        #[serde(rename = "PUT")]
        PUT,
        /// 删除数据
        #[serde(rename = "DELETE")]
        DELETE,
    }
}
