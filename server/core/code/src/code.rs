//! 业务码
use std::io;

use serde::{ser::Serializer, Serialize};

/// 错误种类
#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    /// 成功
    #[error("ok")]
    OK = 0,
    /// 未知错误
    #[error("未知错误")]
    UnknownError = 10001,

    // 服务错误
    /// 内部服务错误
    #[error("内部服务错误")]
    InternalServerError = 10100,
    /// 请求异常
    #[error("请求异常")]
    RequestError = 10101,
    /// 请求超时
    #[error("请求超时")]
    RequestTimeout = 10102,
    /// 无效请求参数
    #[error("无效请求参数, err: {0}")]
    InvalidParameterError(String) = 10103,
    /// 配置解析错误
    #[error("配置解析错误")]
    ConfigParseError = 10105,

    // 数据处理异常
    /// Serialize the given data structure as a String of JSON.
    #[error("结构序列化为JSON字符串错误, err: {0}")]
    JsonSerialization(String) = 10150,
    /// Deserialize an instance of type T from a string of JSON text.
    #[error("从JSON文本字符串中反序列化错误, err: {0}")]
    JsonDeserialization(String) = 10151,
    #[error("JSON转换错误, err: {0}")]
    JsonConvert(String) = 10152,
    /// No data available
    #[error("No data available")]
    NoDataAvailable = 10153,
    /// An input/output error
    #[error("An input/output error {0}")]
    IoError(io::Error) = 10154,
    /// A possible error value when converting a String from a UTF-8 byte vector.
    #[error("A possible error value when converting a String from a UTF-8 byte vector.")]
    FromUtf8Error(std::string::FromUtf8Error) = 10155,

    #[error("数据库初始化失败")]
    DbInitError(String) = 10200,
    #[error("数据库连接失败")]
    DbConnectionError(String) = 10201,
    #[error("数据库ping失败")]
    DbConnectionAcquire(String) = 10202,
    #[error("数据库关闭失败")]
    DbCloseError = 10203,
    #[error("查询数据失败, err: {0}")]
    DbQueryError(String) = 10204,
    #[error("未查到数据")]
    DbQueryEmptyError = 10205,
    #[error("添加数据失败, err: {0}")]
    DBAddError(String) = 10206,
    #[error("更新数据失败, err: {0}")]
    DBUpdateError(String) = 10207,
    #[error("删除数据失败, err: {0}")]
    DBDeleteError(String) = 10208,
    #[error("批量删除数据失败, err: {0}")]
    DBBatchDeleteError(String) = 10209,
    #[error("更新数据状态失败")]
    DBUpdateStatusError = 10210,
    #[error("数据已存在")]
    DBDataExistError = 10211,
    #[error("数据已存在子项")]
    DBDataExistChildrenError = 10212,

    // 鉴权
    #[error("未知的验证码")]
    CaptchaNotExist = 10251,
    #[error("验证码已过期, 请刷新重试")]
    CaptchaExpire = 10252,
    #[error("验证码错误")]
    CaptchaInvalid = 10253,
    #[error("账号或密码错误")]
    LoginPasswordError = 10254,
    #[error("获取密匙异常, err: {0}")]
    TokenEncode(String) = 10255,
    #[error("解析密匙异常, err: {0}")]
    TokenDecode(String) = 10256,
    #[error("非法请求")]
    HeadersNotAuthorization = 10257,
    #[error("非法请求")]
    HeadersNotAuthorizationBearer = 10258,
    #[error("获取服务实例失败")]
    InjectAproviderObj = 10259,
    #[error("当前登陆态已失效, 请重新登陆")]
    LoginStatusDisabled = 10260,

    // 文件或目录操作
    #[error("获取目录失败")]
    FsReadDirError = 10301,
    #[error("获取上级目录失败")]
    FsParentDirError = 10302,
    #[error("创建目录失败")]
    FsCreateDir = 10303,
    #[error("读取文件失败")]
    FsReadFileError(String) = 10304,
    #[error("创建文件失败")]
    FsCreateFileError(String) = 10305,
    #[error("写入文件失败")]
    FsWriterFileError(String) = 10306,
    #[error("内置资源读取失败")]
    AssetReadError = 10307,

    // 内部框架错误
    #[error("日志初始化失败")]
    LoggerInitError(String) = 10351,

    /// 自定义错误
    #[error("自定义错误")]
    CustomError = 65535,
    // Other error from higher-level crate, for downcasting
    // Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// 业务码序列化
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Error {
    /// 返回错误码
    pub fn code(&self) -> u16 {
        unsafe {
            let ptr = self as *const Error as *const u16;
            ptr.read_volatile()
        }
    }
    /// 返回错误码信息
    pub fn msg(&self) -> String {
        self.to_string()
    }
}

/// IO 错误转换
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        if err.kind() == io::ErrorKind::UnexpectedEof {
            return Error::NoDataAvailable;
        }
        Error::IoError(err)
    }
}

/// Utf8 错误转换
impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8Error(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_msg_derive::CodeMessage;

    #[test]
    fn test_error_msg() {
        let err = Error::AssetReadError;
        println!("== {}", err);
        assert!(err.to_string() == "内置资源读取失败");
        assert!(err.msg() == "内置资源读取失败");
    }

    #[test]
    fn test_error_code() {
        let mut err = Error::LoggerInitError("0".to_string());
        println!("== {}", err);
        assert!(err.to_string() == "LoggerInitError(0)");
        let code = unsafe {
            let mul_err = &mut err;
            let ptr: *const u16 = mul_err as *mut Error as *const u16;
            ptr.read_volatile()
        };
        println!("== {}", code);
        assert!(code == 200);
    }

    #[test]
    fn test_error_code2() {
        let err = Error::LoggerInitError("0".to_string());
        let code = err.code();
        println!("== {}", code);
        assert!(code == 200);
    }

    #[derive(CodeMessage)]
    enum Error2 {
        #[status(code = 0, msg = "ok")]
        OK,
        /// 未知错误
        #[status(code = 10001, msg = "unknown error")]
        UnknownError,
    }

    #[test]
    fn test_error_code_message() {
        assert!(Error2::UnknownError.code() == 10001);
        assert!(Error2::OK.code() == 0);
    }
}
