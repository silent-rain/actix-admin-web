//! 字典数据管理
use crate::{
    dao::dict_data::DictDataDao,
    dto::dict_data::{AddDictDataReq, GetDictDataListReq, UpdateDictDataReq},
};

use code::{Error, ErrorMsg};
use entity::sys_dict_data;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct DictDataService<'a> {
    dict_data_dao: DictDataDao<'a>,
}

impl<'a> DictDataService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDictDataListReq,
    ) -> Result<(Vec<sys_dict_data::Model>, u64), ErrorMsg> {
        let (results, total) = self.dict_data_dao.list(req).await.map_err(|err| {
            error!("查询字典数据列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询字典数据列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<sys_dict_data::Model, ErrorMsg> {
        let result = self
            .dict_data_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询字典数据信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询字典数据信息失败")
            })?
            .ok_or_else(|| {
                error!("字典数据不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("字典数据不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddDictDataReq) -> Result<sys_dict_data::Model, ErrorMsg> {
        // 查询字典数据是否已存在
        let dict_data = self
            .dict_data_dao
            .info_by_lable(req.dim_id, req.lable.clone())
            .await
            .map_err(|err| {
                error!("查询字典标签失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询字典标签失败")
            })?;
        if dict_data.is_some() {
            error!("字典标签已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("字典标签已存在"));
        }

        let model = sys_dict_data::ActiveModel {
            dim_id: Set(req.dim_id),
            dim_code: Set(req.dim_code),
            lable: Set(req.lable),
            value: Set(req.value),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(sys_dict_data::enums::Status::Enabled as i8),
            ..Default::default()
        };
        let result = self.dict_data_dao.add(model).await.map_err(|err| {
            error!("添加字典数据信息失败, err: {:#?}", err);
            Error::DbAddError
                .into_msg()
                .with_msg("添加字典数据信息失败")
        })?;

        Ok(result)
    }

    /// 更新字典数据
    pub async fn update(&self, id: i32, req: UpdateDictDataReq) -> Result<u64, ErrorMsg> {
        let model = sys_dict_data::ActiveModel {
            id: Set(id),
            lable: Set(req.lable),
            value: Set(req.value),
            sort: Set(req.sort),
            note: Set(req.note),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self.dict_data_dao.update(model).await.map_err(|err| {
            error!("更新字典数据失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新字典数据失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.dict_data_dao.status(id, status).await.map_err(|err| {
            if err == RecordNotUpdated {
                error!("更新字典数据状态失败, 该字典数据不存在");
                return Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新字典数据状态失败, 该字典数据不存在");
            }
            error!("更新字典数据状态失败, err: {:#?}", err);
            Error::DbUpdateError
                .into_msg()
                .with_msg("更新字典数据状态失败")
        })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.dict_data_dao.delete(id).await.map_err(|err| {
            error!("删除字典数据信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除字典数据信息失败")
        })?;

        Ok(result)
    }
}
