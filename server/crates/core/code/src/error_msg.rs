//! 错误码消息

use crate::error::Error;

pub struct ErrorMsg {
    inner: Error,
    code: u16,
    msg: String,
}

impl ErrorMsg {
    /// 返回原始错误
    pub fn into_inner(self) -> Error {
        self.inner
    }

    /// 返回错误码
    pub fn code(&self) -> u16 {
        self.code
    }

    /// 返回错误信息
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// 重置错误信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_owned();
        self
    }

    /// 追加错误信息, 在错误码信息的基础上添加新的信息
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg.to_owned());
        self
    }
}

impl Error {
    pub fn into_msg(self) -> ErrorMsg {
        ErrorMsg::from(self)
    }
}

impl From<Error> for ErrorMsg {
    fn from(err: Error) -> Self {
        let code = err.code();
        let msg = err.msg();
        ErrorMsg {
            inner: err,
            code,
            msg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_msg() {
        let msg = ErrorMsg::from(Error::DbAddError);
        assert!(msg.code() == 10208);

        let msg = ErrorMsg::from(Error::DbAddError).with_msg("msg");
        assert!(msg.msg() == "msg");

        let msg = ErrorMsg::from(Error::DbAddError).append_msg("msg");
        assert!(msg.msg() == "添加数据失败, msg");
    }
}
