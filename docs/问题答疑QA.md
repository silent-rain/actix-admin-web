# 问题答疑 Q&A

## 自定义序列化与反序列化

```rust
/// josn 解析器
#[derive(Debug, Clone,Serialize, Deserialize, PartialEq)]
pub struct JsonLayer {
    name: String,
    #[serde(
        rename = "max_level",
        deserialize_with = "utils::level::str_to_level",
        serialize_with = "utils::level::level_to_str"
    )]
    max_level: tracing::Level,
}
```
