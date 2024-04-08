//! 用户表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "perm_user")]
pub struct Model {
    /// 用户ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别, 0:男,1:女,2:保密
    pub gender: i8,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub birthday: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮件
    pub email: Option<String>,
    /// 介绍
    pub intro: Option<String>,
    /// 备注
    pub note: Option<String>,
    /// 密码
    pub password: String,
    /// 状态,0:停用,1:正常
    pub status: i8,
    /// 部门ID
    pub dept_id: Option<i32>,
    /// 创建者
    pub creator: Option<i32>,
    /// 更新者
    pub updater: Option<i32>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::api_token::Entity")]
    ApiToken,
    #[sea_orm(has_many = "super::perm_role_user_rel::Entity")]
    PermUserRoleRel,
}

impl Related<super::api_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiToken.def()
    }
}

impl Related<super::perm_role_user_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermUserRoleRel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
