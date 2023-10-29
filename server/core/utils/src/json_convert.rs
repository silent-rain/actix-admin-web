//! JSON 序列化与反序列化转换
use serde::{Deserialize, Deserializer, Serializer};

/// 反序列化 vec 转 string
fn vec_to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let v: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(serde_json::to_string(&v).unwrap())
}

/// 序列化 i8 转 bool
fn i8_to_bool<S: Serializer>(v: &i8, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_bool(*v != 0)
}

/// 反序列化 bool 转 i8
fn bool_to_i8<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i8, D::Error> {
    let b: bool = Deserialize::deserialize(deserializer)?;
    Ok(if b { 1 } else { 0 })
}
