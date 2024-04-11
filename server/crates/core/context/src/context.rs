//! 上下文管理

use std::cell::{Cell, RefCell};

/// 上下文模型
#[derive(Debug, Clone)]
pub struct Context {
    /// 用户ID
    pub user_id: Cell<i32>,
    /// 用户名称
    pub user_name: RefCell<String>,
    /// 接口请求UUID
    pub request_id: RefCell<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            user_id: Cell::new(0),
            user_name: RefCell::new("".to_owned()),
            request_id: RefCell::new("".to_owned()),
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
