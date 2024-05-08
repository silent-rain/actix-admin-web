//! 库表初始化

use crate::{
    asset::AssetDbTable,
    dao::table::TableDao,
    dto::table::{AddAdminUserReq, TableSql},
};

use code::{Error, ErrorMsg};
use entity::user::user_base;

use nject::injectable;
use tracing::error;
use utils::{asset::EmbedAssetTrait, crypto::sha2_256};

/// 服务层
#[injectable]
pub struct TableService<'a> {
    table_dao: TableDao<'a>,
}

impl<'a> TableService<'a> {
    /// 初始化库表
    pub async fn table(&self, req: AddAdminUserReq) -> Result<user_base::Model, ErrorMsg> {
        let mut data = req.clone();
        // 密码加密
        data.password = sha2_256(&data.password);

        let asset = AssetDbTable;
        let db_sql = asset.to_string("t_db.sql").map_err(|err| {
            error!("数据库资源解析错误, err: {err}");
            Error::AssetParseError
                .into_msg()
                .with_msg("数据库资源解析错误")
        })?;

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

        let table_sql = TableSql {
            db_sql: db_sql.clone(),
            role_sql,
            openapi_sql,
            menu_sql,
        };

        // 初始化库表
        let _ = self.table_dao.init_db(db_sql).await.map_err(|err| {
            error!("初始化数据库失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("初始化数据库失败, 请稍后再试")
        })?;

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
        // 初始化库表
        let result = self.table_dao.table(data, table_sql).await.map_err(|err| {
            error!("初始化数据失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("初始化数据失败")
        })?;

        Ok(result)
    }
}
