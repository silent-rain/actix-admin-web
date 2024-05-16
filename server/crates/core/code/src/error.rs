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
    InternalServerError = 10101,
    /// 请求异常
    #[error("请求异常")]
    RequestError,
    /// 请求超时
    #[error("请求超时")]
    RequestTimeout,
    /// 无效请求参数
    #[error("无效请求参数")]
    InvalidParameterError,
    /// 配置解析错误
    #[error("配置解析错误")]
    ConfigParseError = 10105,

    // 数据处理异常
    /// Serialize the given data structure as a String of JSON.
    #[error("结构序列化为JSON字符串错误, {0}")]
    JsonSerialization(String) = 10150,
    /// Deserialize an instance of type T from a string of JSON text.
    #[error("从JSON文本字符串中反序列化错误, {0}")]
    JsonDeserialization(String) = 10151,
    #[error("JSON转换错误")]
    JsonConvert = 10152,
    /// No data available
    #[error("No data available")]
    NoDataAvailable = 10153,
    /// An input/output error
    #[error("An input/output error, {0}")]
    IoError(io::Error) = 10154,
    /// A possible error value when converting a String from a UTF-8 byte vector.
    #[error("A possible error value when converting a String from a UTF-8 byte vector, {0}")]
    FromUtf8Error(std::string::FromUtf8Error) = 10155,
    #[error("{0}")]
    DeserializerError(String) = 10156,
    #[error("{0}")]
    DateTimeParseError(String) = 10157,

    #[error("数据库初始化失败, {0}")]
    DbInitError(String) = 10200,
    #[error("数据库连接失败, {0}")]
    DbConnectionError(String) = 10202,
    #[error("设置 Time Zone 失败, {0}")]
    DbTimeZoneError(String) = 10203,
    #[error("数据库ping失败, {0}")]
    DbConnectionAcquire(String) = 10204,
    #[error("数据库关闭失败")]
    DbCloseError = 10205,
    #[error("查询数据失败")]
    DbQueryError = 10206,
    #[error("未查到数据")]
    DbQueryEmptyError = 10207,
    #[error("添加数据失败")]
    DbAddError = 10208,
    #[error("批量添加数据失败")]
    DbBatchAddError = 10209,
    #[error("更新数据失败")]
    DbUpdateError = 10210,
    #[error("删除数据失败")]
    DbDeleteError = 10211,
    #[error("批量删除数据失败")]
    DbBatchDeleteError = 10212,
    #[error("更新数据状态失败")]
    DbUpdateStatusError = 10213,
    #[error("数据已存在")]
    DbDataExistError = 10214,
    #[error("数据已存在子项")]
    DbDataExistChildrenError = 10215,

    // 鉴权
    #[error("未知的验证码")]
    CaptchaNotExist = 10251,
    #[error("验证码已过期, 请刷新重试")]
    CaptchaExpire = 10252,
    #[error("验证码错误")]
    CaptchaInvalid = 10253,
    #[error("账号或密码错误")]
    LoginPasswordError = 10254,
    #[error("用户已被禁用")]
    LoginUserDisableError = 10255,
    #[error("获取密匙异常")]
    TokenEncode = 10256,
    #[error("鉴权解析失败, err: {0}")]
    TokenDecode(String) = 10257,
    #[error("获取鉴权标识失败")]
    HeadersNotAuthorization = 10258,
    #[error("获取鉴权前缀失败")]
    HeadersNotAuthorizationBearer = 10259,
    #[error("获取服务实例失败")]
    InjectAproviderObj = 10260,
    #[error("当前登陆态已失效, 请重新登陆")]
    LoginStatusDisabled = 10261,
    #[error("用户添加失败")]
    UserAddError = 10262,
    #[error("获取鉴权标识失败")]
    HeadersNotAuthorizationPassphrase = 10263,
    #[error("Illegal Request")]
    AuthIllegalRequest = 10266,

    #[error("生成用户分享码失败")]
    UserShareCore = 10264,

    #[error("数据库初始化失败, 管理员已存在无需重复初始化")]
    DbInitByAdminExistError = 10265,

    // 工具箱
    #[error("User-Agent解析错误, {0}")]
    UserAgentParserError(String) = 10281,
    #[error("Uuid解析失败, {0}")]
    UuidParseError(String) = 10282,
    #[error("调度任务移除解析失败, {0}")]
    ScheduleRemoveError(String) = 10283,
    #[error("Get Schedule Instance Error")]
    ScheduleInstance = 10284,

    #[error("未找到资源")]
    AssetNotFound = 10290,
    #[error("资源解析错误")]
    AssetParseError = 10291,
    #[error("缓存不存在")]
    CacheNotFound = 10292,
    #[error("Casbin 策略执行失败, {0}")]
    CasbinEnforceError(String),
    #[error("No access permission")]
    CasbinNoAccessPermission,

    // 文件或目录操作
    #[error("获取目录失败")]
    FsReadDirError = 10301,
    #[error("获取上级目录失败")]
    FsParentDirError = 10302,
    #[error("创建目录失败")]
    FsCreateDir = 10303,
    #[error("读取文件失败, {0}")]
    FsReadFileError(String) = 10304,
    #[error("创建文件失败, {0}")]
    FsCreateFileError(String) = 10305,
    #[error("写入文件失败, {0}")]
    FsWriterFileError(String) = 10306,

    #[error("{0}")]
    SchedulerInitError(String) = 10401,
    #[error("获取实例错误")]
    SchedulerInstanceError = 10402,

    // 内部框架错误
    #[error("日志初始化失败, {0}")]
    LoggerInitError(String) = 10351,

    /// 自定义错误
    #[error("自定义错误")]
    CustomError = 65535,
    // Other error from higher-level crate, for downcasting
    // Other(Box<dyn std::error::Error + Send + Sync + 'static>),
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

/// 业务码序列化
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
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
    fn test_error_code() {
        let mut err = Error::LoggerInitError("0".to_string());
        println!("== {}", err);
        assert!(err.to_string() == "日志初始化失败, 0");
        let code = unsafe {
            let mul_err = &mut err;
            let ptr: *const u16 = mul_err as *mut Error as *const u16;
            ptr.read_volatile()
        };
        println!("== {}", code);
        assert!(code == 10351);
    }

    #[test]
    fn test_error_code2() {
        let err = Error::LoggerInitError("0".to_string());
        let code = err.code();
        println!("== {}", code);
        assert!(code == 10351);
    }
}
