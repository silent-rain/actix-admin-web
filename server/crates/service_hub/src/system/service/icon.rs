//! ICON图标

use crate::system::{
    dao::icon::IconDao,
    dto::icon::{AddIconReq, GetIconListReq, GetIconListRsp, GetIconRsp, UpdateIconReq},
};

use code::{Error, ErrorMsg};
use entity::sys_icon;
use utils::json::struct_to_struct;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct IconService<'a> {
    captcha_dao: IconDao<'a>,
}

impl<'a> IconService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: GetIconListReq) -> Result<(Vec<GetIconListRsp>, u64), ErrorMsg> {
        let (results, total) = self.captcha_dao.list(req).await.map_err(|err| {
            error!("查询ICON图标列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询ICON图标列表失败")
        })?;

        let results: Vec<GetIconListRsp> = struct_to_struct(&results).map_err(|err| {
            error!("ICON图标列表转换失败, err: {:#?}", err);
            Error::JsonConvert
                .into_msg()
                .with_msg("ICON图标列表转换失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<GetIconRsp, ErrorMsg> {
        let result = self
            .captcha_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询ICON图标信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询ICON图标信息失败")
            })?
            .ok_or_else(|| {
                error!("ICON图标不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("ICON图标不存在")
            })?;

        let base_img = String::from_utf8_lossy(&result.base_img.clone()).to_string();

        let mut result: GetIconRsp = struct_to_struct(&result).map_err(|err| {
            error!("ICON图标列表转换失败, err: {:#?}", err);
            Error::JsonConvert
                .into_msg()
                .with_msg("ICON图标列表转换失败")
        })?;

        result.base_img = base_img;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddIconReq) -> Result<sys_icon::Model, ErrorMsg> {
        let model = sys_icon::ActiveModel {
            name: Set(req.name),
            base_img: Set(req.base_img.as_bytes().to_vec()),
            category: Set(req.category),
            note: Set(req.note),
            ..Default::default()
        };
        let result = self.captcha_dao.add(model).await.map_err(|err| {
            error!("添加ICON图标信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加ICON图标信息失败")
        })?;

        Ok(result)
    }

    /// 更新配置
    pub async fn update(&self, id: i32, req: UpdateIconReq) -> Result<u64, ErrorMsg> {
        let model = sys_icon::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            base_img: Set(req.base_img.as_bytes().to_vec()),
            category: Set(req.category),
            note: Set(req.note),
            ..Default::default()
        };

        let result = self.captcha_dao.update(model).await.map_err(|err| {
            error!("更新ICON图标失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新ICON图标失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.captcha_dao.delete(id).await.map_err(|err| {
            error!("删除ICON图标信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除ICON图标信息失败")
        })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self.captcha_dao.batch_delete(ids).await.map_err(|err| {
            error!("批量删除ICON图标信息失败, err: {:#?}", err);
            Error::DbBatchDeleteError
                .into_msg()
                .with_msg("批量删除ICON图标信息失败")
        })?;

        Ok(result)
    }
}
