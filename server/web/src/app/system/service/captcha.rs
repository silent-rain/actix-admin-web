//! 验证码

use crate::app::system::{dao::captcha::CaptchaDao, dto::captcha::CaptchaListReq};

use code::Error;
use entity::sys_captcha;

use nject::injectable;

/// 服务
#[injectable]
pub struct CaptchaService<'a> {
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> CaptchaService<'a> {
    /// 获取列表数据
    pub async fn list(&self, req: CaptchaListReq) -> Result<(Vec<sys_captcha::Model>, u64), Error> {
        let (results, total) = self
            .captcha_dao
            .list(req)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, uuid: String) -> Result<sys_captcha::Model, Error> {
        let result = self
            .captcha_dao
            .info(uuid)
            .await
            .map_err(|err| Error::DbQueryError(err.to_string()))?
            .ok_or(Error::DbQueryEmptyError)?;
        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, data: sys_captcha::Model) -> Result<sys_captcha::Model, Error> {
        let result = self
            .captcha_dao
            .add(data)
            .await
            .map_err(|err| Error::DBAddError(err.to_string()))?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = self
            .captcha_dao
            .delete(id)
            .await
            .map_err(|err| Error::DBDeleteError(err.to_string()))?;
        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, Error> {
        let result = self
            .captcha_dao
            .batch_delete(ids)
            .await
            .map_err(|err| Error::DBBatchDeleteError(err.to_string()))?;
        Ok(result)
    }
}
