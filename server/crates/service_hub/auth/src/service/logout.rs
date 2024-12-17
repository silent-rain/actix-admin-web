//! 登出

use log::UserLoginDao;
use user::cached::UserCached;

use code::ErrorMsg;
use entity::user::user_login_log;

use nject::injectable;
use tracing::error;

/// 服务层
#[injectable]
pub struct Logoutervice {
    user_login_dao: UserLoginDao,
}

impl Logoutervice {
    /// 登出
    pub async fn logout(&self, user_id: i32, user_login_id: i32) -> Result<(), ErrorMsg> {
        // 移除用户鉴权缓存
        UserCached::remove_user_api_auth(user_id).await;

        // 更新登陆日志状态
        self.user_login_dao
            .status(user_login_id, user_login_log::enums::Status::Logout as i8)
            .await
            .map_err(|err| {
                error!("登出失败, err: {:#?}", err);
                code::Error::DbAddError.into_msg().with_msg("登出失败")
            })?;
        Ok(())
    }
}
