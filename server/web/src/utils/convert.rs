//! 数据转换

use code::Error;

use tracing::error;

/// 将一个结构体转换为另一个结构体
/// 将一个结构体转换为另一个结构体
pub fn convert_struct<S, T>(src: &S) -> Result<T, Error>
where
    S: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    // 转换为JSON字符串
    let data = serde_json::to_string(src).map_err(|err| {
        error!("转换为JSON字符串失败, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })?;

    // 将JSON字符串反序列化为结构体
    let target: T = serde_json::from_str(&data).map_err(|err| {
        error!("将JSON字符串反序列化为结构体失败, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;
    Ok(target)
}
