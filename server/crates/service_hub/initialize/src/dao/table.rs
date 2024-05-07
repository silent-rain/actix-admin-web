//! 库表初始化

use crate::dto::table::AddAdminUserReq;
use crate::dto::table::TableSql;

use database::DbRepo;
use entity::{
    perm_menu_role_rel, perm_openapi_role_rel, perm_role, perm_user, perm_user_email,
    perm_user_phone, perm_user_role_rel,
    prelude::{PermMenu, PermMenuRoleRel, PermOpenapi, PermOpenapiRoleRel, PermRole, PermUser},
};

use nject::injectable;
use sea_orm::ConnectionTrait;
use sea_orm::ExecResult;
use sea_orm::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, QueryOrder, TransactionTrait,
};

/// 数据访问
#[injectable]
pub struct TableDao<'a> {
    db: &'a dyn DbRepo,
}

impl<'a> TableDao<'a> {
    /// 获取第一个用户即为管理员
    pub async fn admin_user(&self) -> Result<Option<perm_user::Model>, DbErr> {
        let result = PermUser::find()
            .order_by_asc(perm_user::Column::Id)
            .one(self.db.rdb())
            .await?;

        Ok(result)
    }

    /// 获取详情信息
    pub async fn table(
        &self,
        req: AddAdminUserReq,
        table_sql: TableSql,
    ) -> Result<perm_user::Model, DbErr> {
        let txn = self.db.wdb().begin().await?;

        // 初始化角色表
        let _ = self.txn_init_role(&txn, table_sql.role_sql).await?;
        // 初始化菜单表
        let _ = self.txn_init_menu(&txn, table_sql.menu_sql).await?;
        // 初始化OpenApi表
        let _ = self.txn_init_open_api(&txn, table_sql.openapi_sql).await?;

        // 添加管理员
        let admin_user = self.txn_add_admin_user(&txn, req.clone()).await?;
        // 添加管理员手机号码
        let _ = self
            .txn_add_admin_user_phone(&txn, admin_user.id, req.phone)
            .await?;
        // 添加管理员邮箱
        if let Some(email) = req.email {
            let _ = self
                .txn_add_admin_user_email(&txn, admin_user.id, email)
                .await?;
        }

        // 获取管理员角色
        if let Some(admin_role) = self.txn_admin_role(&txn).await? {
            // 添加用户与角色关系
            let _ = self
                .txn_init_user_role_rel(&txn, admin_user.id, admin_role.id)
                .await?;
            // 添加菜单与角色关系
            let _ = self.txn_init_menu_role_rel(&txn, admin_role.id).await?;
            // 添加OpenApi接口角色关系
            let _ = self.txn_init_openapi_role_rel(&txn, admin_role.id).await?;
        }

        // 初始化关系关系

        txn.commit().await?;
        Ok(admin_user)
    }

    /// 初始化库表
    pub async fn init_db(&self, db_sql: String) -> Result<u64, DbErr> {
        let result: ExecResult = self.db.wdb().execute_unprepared(&db_sql).await?;
        Ok(result.rows_affected())
    }

    /// 添加管理员
    async fn txn_add_admin_user(
        &self,
        txn: &DatabaseTransaction,
        req: AddAdminUserReq,
    ) -> Result<perm_user::Model, DbErr> {
        let active_model = perm_user::ActiveModel {
            username: Set(req.username),
            gender: Set(perm_user::enums::Gender::Confidential as i8),
            password: Set(req.password),
            status: Set(perm_user::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result = active_model.insert(txn).await?;
        Ok(result)
    }

    /// 添加管理员手机号码
    async fn txn_add_admin_user_phone(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        phone: String,
    ) -> Result<perm_user_phone::Model, DbErr> {
        let active_model = perm_user_phone::ActiveModel {
            user_id: Set(user_id),
            phone: Set(phone),
            ..Default::default()
        };
        let result = active_model.insert(txn).await?;
        Ok(result)
    }

    /// 添加管理员邮箱
    async fn txn_add_admin_user_email(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        email: String,
    ) -> Result<perm_user_email::Model, DbErr> {
        let active_model = perm_user_email::ActiveModel {
            user_id: Set(user_id),
            email: Set(email),
            ..Default::default()
        };
        let result = active_model.insert(txn).await?;
        Ok(result)
    }

    /// 获取管理员角色
    pub async fn txn_admin_role(
        &self,
        txn: &DatabaseTransaction,
    ) -> Result<Option<perm_role::Model>, DbErr> {
        let result = PermRole::find()
            .order_by_asc(perm_role::Column::Id)
            .one(txn)
            .await?;

        Ok(result)
    }

    /// 初始化角色表
    async fn txn_init_role(
        &self,
        txn: &DatabaseTransaction,
        role_sql: String,
    ) -> Result<u64, DbErr> {
        let result: ExecResult = txn.execute_unprepared(&role_sql).await?;
        Ok(result.rows_affected())
    }

    /// 添加用户与角色关系
    async fn txn_init_user_role_rel(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        role_id: i32,
    ) -> Result<perm_user_role_rel::Model, DbErr> {
        let active_model = perm_user_role_rel::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            ..Default::default()
        };
        let result = active_model.insert(txn).await?;
        Ok(result)
    }

    /// 初始化菜单表
    async fn txn_init_menu(
        &self,
        txn: &DatabaseTransaction,
        menu_sql: String,
    ) -> Result<u64, DbErr> {
        let result: ExecResult = txn.execute_unprepared(&menu_sql).await?;
        Ok(result.rows_affected())
    }

    /// 添加菜单与角色关系
    async fn txn_init_menu_role_rel(
        &self,
        txn: &DatabaseTransaction,
        role_id: i32,
    ) -> Result<i32, DbErr> {
        let menus = PermMenu::find().all(txn).await?;

        let mut models = Vec::new();
        for menu in menus {
            let model = perm_menu_role_rel::ActiveModel {
                menu_id: Set(menu.id),
                role_id: Set(role_id),
                ..Default::default()
            };
            models.push(model);
        }

        let result = PermMenuRoleRel::insert_many(models).exec(txn).await?;
        Ok(result.last_insert_id)
    }

    /// 初始化OpenApi表
    async fn txn_init_open_api(
        &self,
        txn: &DatabaseTransaction,
        openapi_sql: String,
    ) -> Result<u64, DbErr> {
        let result: ExecResult = txn.execute_unprepared(&openapi_sql).await?;
        Ok(result.rows_affected())
    }

    /// 添加OpenApi接口角色关系
    async fn txn_init_openapi_role_rel(
        &self,
        txn: &DatabaseTransaction,
        role_id: i32,
    ) -> Result<i32, DbErr> {
        let open_apis = PermOpenapi::find().all(txn).await?;

        let mut models = Vec::new();
        for open_api in open_apis {
            let model = perm_openapi_role_rel::ActiveModel {
                api_id: Set(open_api.id),
                role_id: Set(role_id),
                ..Default::default()
            };
            models.push(model);
        }

        let result = PermOpenapiRoleRel::insert_many(models).exec(txn).await?;
        Ok(result.last_insert_id)
    }
}
