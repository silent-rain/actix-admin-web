//! 菜单管理
use crate::{
    dao::menu::MenuDao,
    dto::menu::{AddMenuReq, GetMenuListReq, UpdateMenuReq},
    enums::MenuStatus,
};

use code::{Error, ErrorMsg};
use entity::perm_menu;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::list_tree::GenericTree;

/// 服务层
#[injectable]
pub struct MenuService<'a> {
    menu_dao: MenuDao<'a>,
}

impl<'a> MenuService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetMenuListReq,
    ) -> Result<(Vec<perm_menu::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.menu_dao.all().await.map_err(|err| {
                error!("查询所有菜单失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询所有菜单失败")
            });
        }

        let (results, total) = self.menu_dao.list(req).await.map_err(|err| {
            error!("查询菜单列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询菜单列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<perm_menu::Model>>, ErrorMsg> {
        let (results, _total) = self.menu_dao.all().await.map_err(|err| {
            error!("查询菜单列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询菜单列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);
        Ok(results)
    }

    /// 获取子列表数据
    pub async fn children(&self, pid: i32) -> Result<(Vec<perm_menu::Model>, u64), ErrorMsg> {
        let results = self.menu_dao.children(pid).await.map_err(|err| {
            error!("查询子菜单列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询子菜单列表失败")
        })?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<perm_menu::Model, ErrorMsg> {
        let result = self
            .menu_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询菜单信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询菜单信息失败")
            })?
            .ok_or_else(|| {
                error!("菜单不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("菜单不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddMenuReq) -> Result<perm_menu::Model, ErrorMsg> {
        let model = perm_menu::ActiveModel {
            pid: Set(req.pid),
            title: Set(req.title),
            icon: Set(req.icon),
            el_icon: Set(req.el_icon),
            menu_type: Set(req.menu_type as i8),
            open_type: Set(req.open_type as i8),
            path: Set(req.path),
            component: Set(req.component),
            redirect: Set(req.redirect),
            link: Set(req.link),
            link_target: Set(req.link_target.map(|v| v.into())),
            hidden: Set(req.hidden.map(|v| v as i8)),
            root_always_show: Set(req.root_always_show.map(|v| v as i8)),
            permission: Set(req.permission),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(MenuStatus::Enabled as i8),
            ..Default::default()
        };
        let result = self
            .menu_dao
            .add(model)
            .await
            .map_err(|err: sea_orm::prelude::DbErr| {
                error!("添加菜单信息失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加菜单信息失败")
            })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateMenuReq) -> Result<u64, ErrorMsg> {
        let model = perm_menu::ActiveModel {
            id: Set(id),
            pid: Set(req.pid),
            title: Set(req.title),
            icon: Set(req.icon),
            el_icon: Set(req.el_icon),
            menu_type: Set(req.menu_type as i8),
            open_type: Set(req.open_type as i8),
            path: Set(req.path),
            component: Set(req.component),
            redirect: Set(req.redirect),
            link: Set(req.link),
            link_target: Set(req.link_target.map(|v| v.into())),
            hidden: Set(req.hidden.map(|v| v as i8)),
            root_always_show: Set(req.root_always_show.map(|v| v as i8)),
            permission: Set(req.permission),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.menu_dao.update(model).await.map_err(|err| {
            error!("更新菜单失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新菜单失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.menu_dao.status(id, status).await.map_err(|err| {
            error!("更新菜单状态失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新菜单状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let children = self.menu_dao.children(id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("获取所有子列表失败")
        })?;
        if !children.is_empty() {
            error!("请先删除子列表, children count: {:#?}", children.len());
            return Err(Error::DbDataExistChildrenError
                .into_msg()
                .with_msg("请先删除子列表"));
        }

        let result = self.menu_dao.delete(id).await.map_err(|err| {
            error!("删除菜单信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除菜单信息失败")
        })?;

        Ok(result)
    }
}
