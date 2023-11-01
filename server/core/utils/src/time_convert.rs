//! 时间处理工具
use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Deserializer};

// 时间格式
pub const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S.%3f";

/// 将字符串转为 NaiveDateTime
fn str_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    let time_obj = DateTime::parse_from_str(&time_str, DATE_FORMAT).unwrap();
    Ok(time_obj.naive_local())
}

/// 默认有效期为当前时间
fn default_created() -> NaiveDateTime {
    Local::now().naive_local()
}
