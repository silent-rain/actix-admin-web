//! 库表初始化

use crate::{
    asset::{AssetDbTable, AssetDbTableData},
    dao::table::TableDao,
    dto::table::{AddAdminUserReq, TableDataSql},
};

use code::{Error, ErrorMsg};
use entity::user::user_base;

use nject::injectable;
use tracing::error;
use utils::{asset::EmbedAssetTrait, crypto::sha2_256};

/// 服务层
#[injectable]
pub struct TableService {
    table_dao: TableDao,
}

impl TableService {
    /// 初始化库表
    pub async fn table(&self, req: AddAdminUserReq) -> Result<user_base::Model, ErrorMsg> {
        // 初始化库表, 如果已存在则不会重复初始化
        self.init_table().await?;

        // 查询管理员是否存在, 存在则无需初始化
        let admin = self.table_dao.admin_user().await.map_err(|err| {
            error!("查询管理员失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("初始化失败, 请稍后再试")
        })?;

        if admin.is_some() {
            error!("管理员已存在无需重复初始化");
            return Err(Error::DbInitByAdminExistError
                .into_msg()
                .with_msg("管理员已存在无需重复初始化"));
        }

        let result = self.init_table_data(req).await?;
        Ok(result)
    }

    /// 初始化库表
    async fn init_table(&self) -> Result<(), ErrorMsg> {
        let tables: Vec<&str> = vec![
            "table_log.sql",
            "table_user.sql",
            "table_permission.sql",
            "table_organization.sql",
            "table_system.sql",
            "table_schedule.sql",
        ];
        let mut table_content_sql = String::new();
        let asset = AssetDbTable;
        for table in tables {
            let content = asset.to_string(table).map_err(|err| {
                error!("数据库资源解析错误, err: {err}");
                Error::AssetParseError
                    .into_msg()
                    .with_msg("数据库资源解析错误")
            })?;

            table_content_sql.push_str(&content);
            table_content_sql.push('\n');
        }

        // 初始化库表
        let _ = self
            .table_dao
            .init_table(table_content_sql)
            .await
            .map_err(|err| {
                error!("初始化数据库表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("初始化数据库表失败, 请联系开发者")
            })?;
        Ok(())
    }

    /// 初始化表数据
    async fn init_table_data(&self, req: AddAdminUserReq) -> Result<user_base::Model, ErrorMsg> {
        let mut data = req.clone();
        // 密码加密
        data.password = sha2_256(&data.password);

        let asset = AssetDbTableData;
        let role_sql = asset.to_string("t_user_role.sql").map_err(|err| {
            error!("角色表资源解析错误, err: {err}");
            Error::AssetParseError
                .into_msg()
                .with_msg("角色表资源解析错误")
        })?;
        let openapi_sql = asset.to_string("t_perm_openapi.sql").map_err(|err| {
            error!("OpenAPi表资源解析错误, err: {err}");
            Error::AssetParseError
                .into_msg()
                .with_msg("OpenAPi表资源解析错误")
        })?;
        let menu_sql = asset.to_string("t_perm_menu.sql").map_err(|err| {
            error!("菜单表资源解析错误, err: {err}");
            Error::AssetParseError
                .into_msg()
                .with_msg("菜单表源解析错误")
        })?;
        let schedule_job_sql = asset.to_string("t_schedule_job.sql").map_err(|err| {
            error!("任务调度作业表资源解析错误, err: {err}");
            Error::AssetParseError
                .into_msg()
                .with_msg("任务调度作业表源解析错误")
        })?;

        let table_sql = TableDataSql {
            role_sql,
            openapi_sql,
            menu_sql,
            schedule_job_sql,
        };

        // 初始化库表
        let result = self
            .table_dao
            .init_table_data(data, table_sql)
            .await
            .map_err(|err| {
                error!("初始化数据失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("初始化数据失败")
            })?;

        Ok(result)
    }
}
