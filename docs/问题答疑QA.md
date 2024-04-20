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

## async_std 将异步函数包装成同步函数

```rust
use async_std;

fn sync_task(&self, output: Model) {
    async_std::task::block_on(async move {
        self.save_db(output).await;
    })
}
```

## tokio+线程 将异步函数包装成同步函数

```rust
fn sync_task(&self, output: Model) {
std::thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
       // 异步代码
    })
})
.join()
.unwrap();
}
```

## Sea-Orm 设置时区问题

### 修改数据库配置

在 MySQL 服务器端进行配置。
在 MySQL 服务器的配置文件（通常是 my.cnf 或 my.ini）中设置默认时区。

```text
[mysqld]
default-time-zone='+08:00'
```

### 在全局级别设置时区

在全局级别设置时区。

```sql
SET GLOBAL time_zone = '+08:00';
```

### Sea-Orm 配置时区

- 在 MySQL 的 JDBC 连接字符串中，通常使用 serverTimezone 参数来指定时区。
- 然而，在 Rust 中，我们通常使用的是 mysql 或 mysql_async 这样的驱动程序，而不是 JDBC。
- 对于 mysql 和 mysql_async 驱动程序，通常不需要在连接字符串中指定时区参数。这些库通常不支持在连接字符串中直接设置时区参数。通过执行 SQL 命令来设置会话的时区。

```rust
use sea_orm::{Database, DbErr};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let database_url = "mysql://username:password@localhost/dbname";
    let db = Database::connect(database_url).await?;

    // 设置会话时区为北京时间 (UTC+8)
    db.execute(sea_orm::Statement::from_string(
        db.get_database_backend(),
        "SET time_zone = '+08:00'".to_owned(),
    ))
    .await?;

    // ... 执行其他数据库操作 ...

    Ok(())
}
```

## 数据库更新字段自动更新时间

- sqlite 时间更新依旧是一个问题，要么在迁移中对更新时间字段做指定后端屏蔽`ON UPDATE CURRENT_TIMESTAMP`，但 sqlite 依旧不会自动更新；

```rust
.col(
    ColumnDef::new(Column::UpdatedAt)
        .date_time()
        .not_null()
        // Sqlite3 不支持 ON UPDATE CURRENT_TIMESTAMP
        .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP")
        .default(Expr::current_timestamp())
        .comment("更新时间"),
)
```

- 在实体中更新的钩子中，对该字段进行设置为当前时间，支持其他后端的框架；

```rust
#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now());
        Ok(self)
    }
}
```
