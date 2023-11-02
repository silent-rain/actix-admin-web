//! 业务码
use std::io;

use serde::{ser::Serializer, Serialize};
use thiserror;

/// 错误种类
#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    /// 成功
    #[error("ok")]
    OK = 10000,
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
    /// 请求参数解析错误
    #[error("请求参数解析错误")]
    RequestParameterParseError = 10103,
    /// 无效请求参数
    #[error("无效请求参数")]
    InvalidParameterError = 10104,
    /// 配置解析错误
    #[error("配置解析错误")]
    ConfigParseError = 10105,

    // 数据处理异常
    /// 数据编码错误
    #[error("数据编码错误")]
    JsonDataEncodeError = 10150,
    /// 数据解码错误
    #[error("数据解码错误")]
    JsonDataDecodeError = 10151,
    /// 类型转换异常
    #[error("类型转换异常")]
    TypeConvertError = 10152,
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
    #[error("查询数据失败")]
    DbQueryError = 10204,
    #[error("未查找到数据")]
    DbQueryEmptyError = 10205,
    #[error("添加数据失败")]
    DBAddError = 10206,
    #[error("更新数据失败")]
    DBUpdateError = 10207,
    #[error("删除数据失败")]
    DBDeleteError = 10208,
    #[error("批量删除数据失败")]
    DBBatchDeleteError = 10209,
    #[error("更新数据状态失败")]
    DBUpdateStatusError = 10210,
    #[error("数据已存在")]
    DBDataExistError = 10211,
    #[error("数据已存在子项")]
    DBDataExistChildrenError = 10212,

    // 文件或目录操作
    /// 获取目录失败
    #[error("获取目录失败")]
    FsReadDirError = 10301,
    /// 获取上级目录失败
    #[error("获取上级目录失败")]
    FsParentDirError = 10302,
    /// 创建目录失败
    #[error("创建目录失败")]
    FsCreateDir = 10303,
    /// 读取文件失败
    #[error("读取文件失败")]
    FsReadFileError(String) = 10304,
    /// 创建文件失败
    #[error("创建文件失败")]
    FsCreateFileError(String) = 10305,
    /// 写入文件失败
    #[error("写入文件失败")]
    FsWriterFileError(String) = 10306,
    /// 内置资源读取失败
    #[error("内置资源读取失败")]
    AssetReadError = 10307,

    // 内部框架错误
    /// 日志初始化失败
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
}
