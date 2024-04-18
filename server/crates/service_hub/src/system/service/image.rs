//! ICON图片

use crate::system::{
    dao::image::ImageDao,
    dto::icon::{AddIconReq, GetIconListReq, GetIconRsp, UpdateIconReq},
};

use code::{Error, ErrorMsg};
use entity::sys_image;
use utils::json::struct_to_struct;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct ImageService<'a> {
    icon_dao: ImageDao<'a>,
}

impl<'a> ImageService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetIconListReq,
    ) -> Result<(Vec<sys_image::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.icon_dao.list(req).await.map_err(|err| {
            error!("查询ICON图片列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询ICON图片列表失败")
        })?;

        // 屏蔽图片内容
        for item in results.iter_mut() {
            item.base_img = "".as_bytes().to_vec();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<GetIconRsp, ErrorMsg> {
        let result = self
            .icon_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询ICON图片信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询ICON图片信息失败")
            })?
            .ok_or_else(|| {
                error!("ICON图片不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("ICON图片不存在")
            })?;

        let base_img = String::from_utf8_lossy(&result.base_img.clone()).to_string();

        let mut result: GetIconRsp = struct_to_struct(&result).map_err(|err| {
            error!("ICON图片列表转换失败, err: {:#?}", err);
            Error::JsonConvert
                .into_msg()
                .with_msg("ICON图片列表转换失败")
        })?;

        result.base_img = base_img;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddIconReq) -> Result<sys_image::Model, ErrorMsg> {
        let model = sys_image::ActiveModel {
            name: Set(req.name),
            hash_name: Set(req.hash_name),
            base_img: Set(req.base_img.as_bytes().to_vec()),
            img_type: Set(req.img_type),
            note: Set(req.note),
            ..Default::default()
        };
        let result = self.icon_dao.add(model).await.map_err(|err| {
            error!("添加ICON图片信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加ICON图片信息失败")
        })?;

        Ok(result)
    }

    /// 更新配置
    pub async fn update(&self, id: i32, req: UpdateIconReq) -> Result<u64, ErrorMsg> {
        let model = sys_image::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            hash_name: Set(req.hash_name),
            base_img: Set(req.base_img.as_bytes().to_vec()),
            img_type: Set(req.img_type),
            note: Set(req.note),
            ..Default::default()
        };

        let result = self.icon_dao.update(model).await.map_err(|err| {
            error!("更新ICON图片失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新ICON图片失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.icon_dao.delete(id).await.map_err(|err| {
            error!("删除ICON图片信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除ICON图片信息失败")
        })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self.icon_dao.batch_delete(ids).await.map_err(|err| {
            error!("批量删除ICON图片信息失败, err: {:#?}", err);
            Error::DbBatchDeleteError
                .into_msg()
                .with_msg("批量删除ICON图片信息失败")
        })?;

        Ok(result)
    }
}
