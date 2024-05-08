//! 字典维度管理
use crate::{
    dao::dict_dim::DictDimDao,
    dto::dict_dim::{AddDictDimReq, GetDictDimListReq, UpdateDictDimReq},
};

use code::{Error, ErrorMsg};
use entity::sys_dict_dim;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct DictDimService<'a> {
    dict_dim_dao: DictDimDao<'a>,
}

impl<'a> DictDimService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDictDimListReq,
    ) -> Result<(Vec<sys_dict_dim::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.dict_dim_dao.all().await.map_err(|err| {
                error!("查询字典维度列表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询字典维度列表失败")
            });
        }

        let (results, total) = self.dict_dim_dao.list(req).await.map_err(|err| {
            error!("查询字典维度列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询字典维度列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<sys_dict_dim::Model, ErrorMsg> {
        let result = self
            .dict_dim_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询字典维度信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询字典维度信息失败")
            })?
            .ok_or_else(|| {
                error!("字典维度不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("字典维度不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddDictDimReq) -> Result<sys_dict_dim::Model, ErrorMsg> {
        // 查询字典维度名称是否已存在
        let dict_dim = self
            .dict_dim_dao
            .info_by_name(req.name.clone())
            .await
            .map_err(|err| {
                error!("查询字典维度名称失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询字典维度名称失败")
            })?;
        if dict_dim.is_some() {
            error!("字典维度名称已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("字典维度名称已存在"));
        }

        // 查询字典维度编码是否存在
        let dict_dim = self
            .dict_dim_dao
            .info_by_code(req.code.clone())
            .await
            .map_err(|err| {
                error!("查询字典维度编码失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询字典维度编码失败")
            })?;
        if dict_dim.is_some() {
            error!("字典维度编码已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("字典维度编码已存在"));
        }

        let model = sys_dict_dim::ActiveModel {
            name: Set(req.name),
            code: Set(req.code),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(sys_dict_dim::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result = self.dict_dim_dao.add(model).await.map_err(|err| {
            error!("添加字典维度信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加字典维度信息失败")
        })?;

        Ok(result)
    }

    /// 更新字典维度
    pub async fn update(&self, id: i32, req: UpdateDictDimReq) -> Result<u64, ErrorMsg> {
        let model = sys_dict_dim::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            code: Set(req.code),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.dict_dim_dao.update(model).await.map_err(|err| {
            error!("更新字典维度失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新字典维度失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.dict_dim_dao.status(id, status).await.map_err(|err| {
            if err == RecordNotUpdated {
                error!("更新字典维度状态失败, 该字典维度不存在");
                return Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新字典维度状态失败, 该字典维度不存在");
            }
            error!("更新字典维度状态失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新字典维度状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.dict_dim_dao.delete(id).await.map_err(|err| {
            error!("删除字典维度信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除字典维度信息失败")
        })?;

        Ok(result)
    }
}
