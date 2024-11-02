//! WEB日志管理
use crate::{
    dao::web_log::WebLogDao,
    dto::web_log::{AddWebLogInfoReq, GetWebLogListReq},
};

use code::{Error, ErrorMsg};
use entity::log_web;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct WebLogService {
    log_web_dao: WebLogDao,
}

impl WebLogService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetWebLogListReq,
    ) -> Result<(Vec<log_web::Model>, u64), ErrorMsg> {
        let (results, total) = self.log_web_dao.list(req).await.map_err(|err| {
            error!("查询WEB日志列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询WEB日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<log_web::Model, ErrorMsg> {
        let result = self
            .log_web_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询WEB日志信息失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询WEB日志信息失败")
            })?
            .ok_or_else(|| {
                error!("WEB日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("WEB日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(&self, req: AddWebLogInfoReq) -> Result<log_web::Model, ErrorMsg> {
        let model = log_web::ActiveModel {
            user_id: Set(req.user_id),
            username: Set(req.username),
            request_id: Set(req.request_id),
            os_type: Set(req.os_type as i8),
            error_type: Set(req.error_type as i8),
            level: Set(req.level),
            caller_line: Set(req.caller_line),
            url: Set(req.url),
            msg: Set(req.msg),
            stack: Set(req.stack),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.log_web_dao.add(model).await.map_err(|err| {
            error!("添加WEB日志信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("添加WEB日志信息失败")
        })?;

        Ok(result)
    }
}
