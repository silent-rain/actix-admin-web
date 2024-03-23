//! JSON 序列化与反序列化转换

use code::Error;

use serde::{Deserialize, Deserializer, Serializer};
use tracing::error;

/// 反序列化 vec 转 string
pub fn vec_to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let v: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(serde_json::to_string(&v).unwrap())
}

/// 序列化 i8 转 bool
pub fn i8_to_bool<S: Serializer>(v: &i8, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_bool(*v != 0)
}

/// 反序列化 bool 转 i8
pub fn bool_to_i8<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i8, D::Error> {
    let b: bool = Deserialize::deserialize(deserializer)?;
    Ok(if b { 1 } else { 0 })
}

/// 将一个结构体转换为另一个结构体
pub fn struct_to_struct<S, T>(src: &S) -> Result<T, Error>
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
