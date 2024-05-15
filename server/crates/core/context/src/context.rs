//! 上下文管理

use std::cell::{Cell, RefCell};

/// 接口鉴权类型
#[derive(Debug, Clone)]
pub enum ApiAuthType {
    System,
    Openapi,
}

/// 上下文模型
#[derive(Debug, Clone)]
pub struct Context {
    /// 用户ID
    pub user_id: Cell<i32>,
    /// 用户名称
    pub user_name: RefCell<String>,
    /// 接口请求UUID
    pub request_id: RefCell<String>,
    /// 接口鉴权类型
    pub api_auth_type: RefCell<Option<ApiAuthType>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            user_id: Cell::new(0),
            user_name: RefCell::new("".to_owned()),
            request_id: RefCell::new("".to_owned()),
            api_auth_type: RefCell::new(None),
        }
    }
}

/// 用户信息传递
impl Context {
    /// 获取用户ID
    pub fn get_user_id(&self) -> i32 {
        self.user_id.get()
    }
    /// 设置用户ID
    pub fn set_user_id(&self, user_id: i32) {
        self.user_id.set(user_id)
    }
    /// 获取用户昵称
    pub fn get_user_name(&self) -> String {
        self.user_name.clone().into_inner()
    }
    /// 设置用户昵称
    pub fn set_user_name(&mut self, user_name: String) {
        let mut x = self.user_name.borrow_mut();
        *x = user_name;
    }

    /// 获取接口鉴权类型
    pub fn get_api_auth_type(&self) -> Option<ApiAuthType> {
        self.api_auth_type.clone().into_inner()
    }
    /// 设置接口鉴权类型
    pub fn set_api_auth_type(&mut self, api_auth_type: ApiAuthType) {
        let mut x = self.api_auth_type.borrow_mut();
        *x = Some(api_auth_type);
    }

    /// 设置接口请求UUID
    pub fn set_request_id(&mut self, request_id: String) {
        let mut x = self.request_id.borrow_mut();
        *x = request_id;
    }

    /// 获取接口请求UUID
    pub fn get_request_id(&self) -> String {
        self.request_id.clone().into_inner()
    }
}
