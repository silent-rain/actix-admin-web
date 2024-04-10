//! 验证码

use crate::{
    constant::CAPTCHA_EXPIRE,
    system::{
        dao::captcha::CaptchaDao,
        dto::captcha::{AddCaptchaResp, GetCaptchaListReq},
    },
};

use code::{Error, ErrorMsg};
use entity::sys_captcha;
use utils::captcha::generate_captcha;

use nject::injectable;
use sea_orm::Set;
use tracing::{error, warn};
use uuid::Uuid;

/// 服务层
#[injectable]
pub struct CaptchaService<'a> {
    captcha_dao: CaptchaDao<'a>,
}

impl<'a> CaptchaService<'a> {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetCaptchaListReq,
    ) -> Result<(Vec<sys_captcha::Model>, u64), ErrorMsg> {
        let (results, total) = self.captcha_dao.list(req).await.map_err(|err| {
            error!("查询验证码列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询验证码列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<sys_captcha::Model, ErrorMsg> {
        let result = self
            .captcha_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询验证码信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询验证码信息失败")
            })?
            .ok_or_else(|| {
                error!("验证码不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("验证码不存在")
            })?;

        Ok(result)
    }

    /// 通过captcha_id获取详情信息
    pub async fn info_by_captcha_id(
        &self,
        captcha_id: String,
    ) -> Result<sys_captcha::Model, ErrorMsg> {
        let result = self
            .captcha_dao
            .info_by_captcha_id(captcha_id)
            .await
            .map_err(|err| {
                error!("查询验证码信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询验证码信息失败")
            })?
            .ok_or_else(|| {
                error!("验证码不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("验证码不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self) -> Result<AddCaptchaResp, ErrorMsg> {
        // 生成验证码
        let (captcha, base_img) = generate_captcha();
        let captcha_id = Uuid::new_v4().to_string();
        let expire = CAPTCHA_EXPIRE;

        let model = sys_captcha::ActiveModel {
            captcha_id: Set(captcha_id),
            captcha: Set(captcha.clone()),
            base_img: Set(base_img.clone().into_bytes()),
            expire: Set(expire),
            ..Default::default()
        };
        let result = self.captcha_dao.add(model).await.map_err(|err| {
            error!("添加验证码信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("添加验证码信息失败")
        })?;

        let result = AddCaptchaResp {
            captcha_id: result.captcha_id,
            base_img,
            expire: result.expire,
            created_at: result.created_at,
        };
        // TODO 后期调整日志级别
        warn!(
            "Generate verification code, captcha_id: {} captcha: {}",
            result.captcha_id, captcha
        );
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self.captcha_dao.delete(id).await.map_err(|err| {
            error!("删除验证码信息失败, err: {:#?}", err);
            Error::DbDeleteError
                .into_msg()
                .with_msg("删除验证码信息失败")
        })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self.captcha_dao.batch_delete(ids).await.map_err(|err| {
            error!("批量删除验证码信息失败, err: {:#?}", err);
            Error::DbBatchDeleteError
                .into_msg()
                .with_msg("批量删除验证码信息失败")
        })?;

        Ok(result)
    }
}
