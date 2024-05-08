//! 图片

use std::io::Read;

use crate::{
    dao::image::ImageDao,
    dto::image::{GetIconListReq, UpdateIconReq, UploadFileForm, UploadFilesForm},
};

use code::{Error, ErrorMsg};
use entity::sys_image;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use uuid::Uuid;

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
        let (results, total) = self.icon_dao.list(req).await.map_err(|err| {
            error!("查询图片列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询图片列表失败")
        })?;

        // 屏蔽图片内容
        // for item in results.iter_mut() {
        //     item.base_img = "".as_bytes().to_vec();
        // }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<sys_image::Model, ErrorMsg> {
        let result = self
            .icon_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询图片信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询图片信息失败")
            })?
            .ok_or_else(|| {
                error!("图片不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("图片不存在")
            })?;

        Ok(result)
    }

    /// 通过hash值获取详情数据
    pub async fn info_by_hash(&self, hash_name: String) -> Result<sys_image::Model, ErrorMsg> {
        let result = self
            .icon_dao
            .info_by_hash(hash_name)
            .await
            .map_err(|err| {
                error!("获取图片失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("获取图片失败")
            })?
            .ok_or_else(|| {
                error!("图片不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("图片不存在")
            })?;

        Ok(result)
    }

    /// 上传图片
    pub async fn upload_file(&self, form: UploadFileForm) -> Result<sys_image::Model, ErrorMsg> {
        let name = form.file.file_name.map_or("".to_owned(), |v| v);
        let img_type = form
            .file
            .content_type
            .map_or("".to_owned(), |v| v.to_string());
        let base_img = form.file.file.bytes().map(|v| v.unwrap()).collect();
        let img_size = form.file.size as i32;
        let hash_name = Uuid::new_v4().to_string().replace('-', "");

        let model = sys_image::ActiveModel {
            name: Set(name),
            hash_name: Set(hash_name),
            base_img: Set(base_img),
            img_type: Set(img_type),
            img_size: Set(img_size),
            ..Default::default()
        };

        let result = self.icon_dao.add(model).await.map_err(|err| {
            error!("传图片信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("传图片信息失败")
        })?;

        Ok(result)
    }

    /// 批量上传图片
    pub async fn upload_files(&self, form: UploadFilesForm) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for file in form.files {
            let name = file.file_name.map_or("".to_owned(), |v| v);
            let img_type = file.content_type.map_or("".to_owned(), |v| v.to_string());
            let base_img = file.file.bytes().map(|v| v.unwrap()).collect();
            let img_size = file.size as i32;
            let hash_name = Uuid::new_v4().to_string().replace('-', "");

            let model = sys_image::ActiveModel {
                name: Set(name),
                hash_name: Set(hash_name),
                base_img: Set(base_img),
                img_type: Set(img_type),
                img_size: Set(img_size),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self.icon_dao.batch_add(models).await.map_err(|err| {
            error!("批量上传图片失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("批量上传图片失败")
        })?;

        Ok(result)
    }

    /// 更新图片
    pub async fn update(&self, id: i32, req: UpdateIconReq) -> Result<u64, ErrorMsg> {
        let model = sys_image::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.icon_dao.update(model).await.map_err(|err| {
            error!("更新图片失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新图片失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.icon_dao.delete(id).await.map_err(|err| {
            error!("删除图片信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除图片信息失败")
        })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self.icon_dao.batch_delete(ids).await.map_err(|err| {
            error!("批量删除图片信息失败, err: {:#?}", err);
            Error::DbBatchDeleteError
                .into_msg()
                .with_msg("批量删除图片信息失败")
        })?;

        Ok(result)
    }
}
