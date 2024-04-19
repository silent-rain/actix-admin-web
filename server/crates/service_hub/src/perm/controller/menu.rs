//! 菜单管理

use crate::{
    inject::AInjectProvider,
    perm::{
        dto::menu::{AddMenuReq, GetMenuListReq, UpdateMenuReq, UpdateMenuStatusReq},
        service::menu::MenuService,
    },
};

use actix_validator::{Json, Query};
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};

/// 控制器
pub struct MenuController;

impl MenuController {
    /// 获取菜单列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetMenuListReq>,
    ) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.list(req.into_inner()).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取菜单树列表
    pub async fn tree(provider: Data<AInjectProvider>) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.tree().await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取子菜单树列表
    pub async fn children(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.children(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 获取菜单信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加菜单
    pub async fn add(provider: Data<AInjectProvider>, data: Json<AddMenuReq>) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新菜单
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateMenuReq>,
    ) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新菜单状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateMenuStatusReq>,
    ) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.status(*id, data.status.clone() as i8).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除菜单
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let menu_service: MenuService = provider.provide();
        let resp = menu_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}
