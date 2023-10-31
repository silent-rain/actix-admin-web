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

## 将异步函数包装成同步函数

```rust
use async_std;

fn sync_task(&self, output: Model) {
    async_std::task::block_on(async move {
        self.save_db(output).await;
    })
}
```

```rust
fn sync_task(&self, output: Model) {
    tokio::spawn(async move {
        Self::save_db(output).await;
    });
}
```
