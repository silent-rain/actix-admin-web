//! 用户角色关联关系管理

use database::DBRepo;
use entity::perm_user_role_rel;
use entity::prelude::PermUserRoleRel;

use nject::injectable;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

#[injectable]
pub struct UserRoleRelDao<'a> {
    db: &'a dyn DBRepo,
}

impl<'a> UserRoleRelDao<'a> {
    /// 获取数据列表
    pub async fn list(&self, user_id: i32) -> Result<(Vec<perm_user_role_rel::Model>, u64), DbErr> {
        let results = PermUserRoleRel::find()
            .filter(perm_user_role_rel::Column::UserId.eq(user_id))
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 添加关联关系
    pub async fn add(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<perm_user_role_rel::Model, DbErr> {
        let pear = perm_user_role_rel::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            ..Default::default() // all other attributes are `NotSet`
        };

        pear.insert(self.db.wdb()).await
    }

    /// 通过用户ID删除关联关系
    pub async fn delete_by_user_id(&self, user_id: i32) -> Result<u64, DbErr> {
        let result = PermUserRoleRel::delete_many()
            .filter(perm_user_role_rel::Column::UserId.eq(user_id))
            .exec(self.db.wdb())
            .await?;
        Ok(result.rows_affected)
    }
}
