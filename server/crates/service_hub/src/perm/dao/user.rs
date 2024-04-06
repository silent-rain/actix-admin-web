//! 用户管理
use crate::perm::dto::user::GetUserListReq;

use database::{DbRepo, Pagination};
use entity::{
    perm_role, perm_user, perm_user_role_rel,
    prelude::{PermRole, PermUser, PermUserRoleRel},
};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set, TransactionTrait,
};

/// 数据访问
#[injectable]
pub struct UserDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> UserDao<'a> {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let results = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(&self, req: GetUserListReq) -> Result<(Vec<perm_user::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = PermUser::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(perm_user::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(perm_user::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.rdb()).await?;

        let results = states
            .order_by_desc(perm_user::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.rdb())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find_by_id(id).one(self.db.rdb()).await
    }

    /// 根据手机号码获取详情信息
    pub async fn info_by_phone(&self, phone: String) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find()
            .filter(perm_user::Column::Phone.contains(phone))
            .one(self.db.rdb())
            .await
    }

    /// 根据邮箱获取详情信息
    pub async fn info_by_email(&self, email: String) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find()
            .filter(perm_user::Column::Email.contains(email))
            .one(self.db.rdb())
            .await
    }

    /// 根据手机号/邮箱获取详情信息
    pub async fn info_by_username(
        &self,
        username: String,
    ) -> Result<Option<perm_user::Model>, DbErr> {
        PermUser::find()
            .filter(
                Condition::any()
                    .add(perm_user::Column::Phone.eq(username.clone()))
                    .add(perm_user::Column::Email.eq(username)),
            )
            .one(self.db.rdb())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: perm_user::ActiveModel,
    ) -> Result<perm_user::Model, DbErr> {
        active_model.insert(self.db.wdb()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: perm_user::ActiveModel) -> Result<u64, DbErr> {
        // let pear = perm_user::ActiveModel {
        //     nickname: Set(data.nickname),
        //     gender: Set(data.gender),
        //     age: Set(data.age),
        //     phone: Set(data.phone),
        //     password: Set(data.password),
        //     status: Set(data.status),
        //     ..Default::default()
        // };

        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermUser::update_many()
            .set(active_model)
            .filter(perm_user::Column::Id.eq(id))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = PermUser::delete_by_id(id).exec(self.db.wdb()).await?;
        Ok(result.rows_affected)
    }

    /// 指定字段删除
    pub async fn delete_by_name(&self, username: String) -> Result<u64, DbErr> {
        let result = PermUser::delete_many()
            .filter(perm_user::Column::Username.contains(&username))
            .exec(self.db.wdb())
            .await?;

        Ok(result.rows_affected)
    }
}

impl<'a> UserDao<'a> {
    /// 添加用户及对应用户的角色
    pub async fn add_user(
        &self,
        data: perm_user::ActiveModel,
        add_role_ids: Vec<i32>,
    ) -> Result<perm_user::Model, DbErr> {
        let txn = self.db.wdb().begin().await?;

        // 添加用户
        let user = self.txn_add_user(&txn, data).await?;
        let user_id = user.id;

        // 添加批量角色
        let _ = self.txn_add_user_roles(&txn, user_id, add_role_ids).await?;

        txn.commit().await?;
        Ok(user)
    }

    /// 更新用户及对应用户的角色
    pub async fn update_user(
        &self,
        active_model: perm_user::ActiveModel,
        add_role_ids: Vec<i32>,
        del_role_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        let user_id: i32 = *(active_model.id.clone().as_ref());
        let txn = self.db.wdb().begin().await?;

        // 更新用户
        let _ = self.txn_update_user(&txn, active_model).await?;
        // 添加批量角色
        let _ = self.txn_add_user_roles(&txn, user_id, add_role_ids).await?;
        // 删除批量角色
        let _ = self.txn_del_user_roles(&txn, user_id, del_role_ids).await?;

        txn.commit().await?;
        Ok(())
    }

    /// 添加用户
    async fn txn_add_user(
        &self,
        txn: &DatabaseTransaction,
        data: perm_user::ActiveModel,
    ) -> Result<perm_user::Model, DbErr> {
        data.insert(txn).await
    }

    /// 更新用户
    async fn txn_update_user(
        &self,
        txn: &DatabaseTransaction,
        active_model: perm_user::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = PermUser::update_many()
            .set(active_model)
            .filter(perm_user::Column::Id.eq(id))
            .exec(txn)
            .await?;
        Ok(result.rows_affected)
    }

    /// 添加批量角色
    async fn txn_add_user_roles(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<i32, DbErr> {
        if role_ids.is_empty() {
            return Ok(0);
        }
        let mut user_ids = Vec::new();
        for role_id in role_ids {
            let model = perm_user_role_rel::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            user_ids.push(model)
        }

        let result = PermUserRoleRel::insert_many(user_ids).exec(txn).await?;
        Ok(result.last_insert_id)
    }

    /// 删除批量角色
    async fn txn_del_user_roles(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<u64, DbErr> {
        if role_ids.is_empty() {
            return Ok(0);
        }

        let result = PermUserRoleRel::delete_many()
            .filter(perm_user_role_rel::Column::UserId.eq(user_id))
            .filter(perm_user_role_rel::Column::RoleId.is_in(role_ids))
            .exec(txn)
            .await?;
        Ok(result.rows_affected)
    }
}

impl<'a> UserDao<'a> {
    // TODO 关联关系待修复
    /// 通过用户ID获取角色列表
    pub async fn roles(&self, user_id: i32) -> Result<(Vec<perm_role::Model>, u64), DbErr> {
        let results = PermRole::find()
            .left_join(PermUserRoleRel)
            .filter(perm_user_role_rel::Column::UserId.eq(user_id))
            .order_by_asc(perm_user::Column::Id)
            .all(self.db.rdb())
            .await?;
        let total = results.len() as u64;

        Ok((results, total))
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::DbBackend;

    use super::*;

    #[test]
    fn test_role_list() {
        let result = PermRole::find()
            .select_only()
            .columns([perm_user::Column::Id])
            .left_join(PermUserRoleRel)
            .filter(perm_user_role_rel::Column::UserId.eq(10))
            .order_by_asc(perm_user::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "perm_user"."id" FROM "perm_user" LEFT JOIN "perm_user_role_rel" ON "perm_user"."id" = "perm_user_role_rel"."role_id" WHERE "perm_user_role_rel"."user_id" = 10 ORDER BY "perm_user"."id" ASC"#;

        assert_eq!(result, sql);
    }
}