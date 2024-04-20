//! 时间处理工具
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{de, Deserialize, Deserializer};

/// 时间格式
pub const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S.%3f";

/// 将字符串转为 NaiveDateTime
pub fn str_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    let naive = NaiveDateTime::parse_from_str(&time_str, DATE_FORMAT).map_err(de::Error::custom)?;
    Ok(naive)
}

/// 将字符串转为 NaiveDateTime
///
/// https://docs.rs/serde/latest/serde/de/trait.Error.html
///
/// format time:
pub fn str_to_local_date_time<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    let naive =
        NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S").map_err(de::Error::custom)?;
    let date_time: DateTime<Local> = match Local.from_local_datetime(&naive) {
        chrono::offset::LocalResult::Single(v) => v,
        _ => return Err(de::Error::custom("time parse failed")),
    };
    Ok(date_time)
}

/// 默认有效期为当前时间
pub fn default_naive_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

/// 默认有效期为当前时间
pub fn default_local_date_time() -> DateTime<Local> {
    Local::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_str_to_local_date_time() {
        #[derive(Debug, Deserialize)]
        pub struct AddUserTokenReq {
            #[serde(
                rename = "expire",
                deserialize_with = "str_to_local_date_time",
                default = "default_local_date_time"
            )]
            pub expire: DateTime<Local>,
        }
        let value = json!({"expire": "2023-12-02 00:00:00"});
        let result: AddUserTokenReq = serde_json::from_value(value).expect("时间解析失败");
        println!("result: {:#?}", result);
    }
}
