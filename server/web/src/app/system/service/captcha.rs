//! 验证码

use crate::app::system::{
    dao::captcha::CaptchaDao,
    dto::captcha::{AddCaptchaReq, CaptchaListReq},
};

use code::Error;
use entity::sys_captcha;

use nject::injectable;
use sea_orm::DbErr::RecordNotFound;
use tracing::error;

/// 服务
#[injectable]
pub struct CaptchaService<'a> {
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> CaptchaService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: CaptchaListReq) -> Result<(Vec<sys_captcha::Model>, u64), Error> {
        let (results, total) = self.captcha_dao.list(req).await.map_err(|err| {
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<sys_captcha::Model>, Error> {
        let result = self.captcha_dao.info(id).await.map_err(|err| {
            if let RecordNotFound(err) = err {
                error!("未查找到数据, error: {err:#?}");
                return Error::DbQueryEmptyError;
            }
            error!("查询数据失败, error: {err:#?}");
            Error::DbQueryError(err.to_string())
        })?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: AddCaptchaReq) -> Result<sys_captcha::Model, Error> {
        let result = self.captcha_dao.add(data).await.map_err(|err| {
            error!("添加数据失败, error: {err:#?}");
            Error::DBAddError
        })?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self.captcha_dao.delete(id).await.map_err(|err| {
            error!("删除数据失败, error: {err:#?}");
            Error::DBDeleteError
        })?;
        Ok(result)
    }
}