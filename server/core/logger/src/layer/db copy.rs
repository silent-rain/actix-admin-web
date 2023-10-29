//! 数据库日志
use std::collections::BTreeMap;
use std::thread;

use config::logger;
use dao::log_system::Dao;
use entity::log_system::Model;

use chrono::Local;
use serde_json::Value;
use tracing::{Event, Metadata, Subscriber};
use tracing_serde::{AsSerde, SerializeMetadata};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// josn 解析器
#[derive(Debug)]
pub struct JsonLayer {
    max_level: tracing::Level,
}

impl Default for JsonLayer {
    fn default() -> JsonLayer {
        JsonLayer {
            max_level: tracing::Level::WARN,
        }
    }
}

impl<S> Layer<S> for JsonLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    /// 用于判断是否启用该层
    fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        // 这里可以根据元数据中的级别、目标或其他信息来决定是否启用该层
        metadata.level() <= &self.max_level
    }

    /// 用于判断是否启用某个事件
    fn event_enabled(&self, event: &Event<'_>, _ctx: Context<'_, S>) -> bool {
        // 这里可以根据事件中的元数据或字段来决定是否启用该事件
        // 例如，如果只想启用包含 message 字段的事件，可以这样写：
        // event.fields().any(|f| f.name() == "message")
        event.metadata().level() <= &self.max_level
    }

    /// 用于处理每个日志事件，也就是每次调用 event! 宏或其简写形式时触发的事件。
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // 当前 span
        let current_span = ctx.current_span();
        let span_id = current_span.id();

        // Covert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // 输出日志
        let span_id = span_id.map(|v| v.into_u64() as u32);
        let metadata = event.metadata().as_serde();
        let output = self.get_output_log(span_id, metadata, &fields);
        let output = match output {
            Some(v) => v,
            None => return,
        };
        // 过滤数据库事件
        if Self::filter_db_log(&output.target, &fields) {
            return;
        }
        self.sync_task(output);
    }

    /// 用于处理每个新创建的 span，也就是每次调用 span! 宏或其简写形式时触发的事件。
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // 基于 field 值来构建我们自己的 JSON 对象
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);

        // 使用之前创建的 newtype 包裹下, 数据私有
        let storage = CustomFieldStorage(fields.clone());

        // 获取内部 span 数据的引用
        let span = ctx.span(id).unwrap();
        // 获取扩展，用于存储我们的 span 数据
        let mut extensions = span.extensions_mut();
        // 存储至 span
        extensions.insert::<CustomFieldStorage>(storage);

        // 输出日志
        let span_id = Some(id.into_u64() as u32);
        let metadata = span.metadata().as_serde();
        let output = self.get_output_log(span_id, metadata, &fields);
        let output = match output {
            Some(v) => v,
            None => return,
        };
        // 过滤数据库事件
        if Self::filter_db_log(&output.target, &fields) {
            return;
        }
        self.sync_task(output);
    }

    /// 事件用于处理每次记录 span 的值，也就是每次调用 record! 宏或其简写形式时触发的事件。
    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // 获取正在记录数据的 span
        let span = ctx.span(id).unwrap();

        // 获取数据的可变引用，该数据是在 on_new_span 中创建的
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let fields: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // 使用我们的访问器老朋友
        let mut visitor = JsonVisitor(fields);
        values.record(&mut visitor);

        // 输出日志
        let span_id = Some(id.into_u64() as u32);
        let metadata = span.metadata().as_serde();
        let output = self.get_output_log(span_id, metadata, fields);
        let output = match output {
            Some(v) => v,
            None => return,
        };
        // 过滤数据库事件
        if Self::filter_db_log(&output.target, fields) {
            return;
        }
        self.sync_task(output);
    }
}

/// 访问者模式
/// 记录 fields 字典
struct JsonVisitor<'a>(&'a mut BTreeMap<String, Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        if let Some(v) = value.downcast_ref::<code::Error>() {
            self.0
                .insert("code".to_string(), serde_json::json!(v.code()));
            self.0
                .insert("code_msg".to_string(), serde_json::json!(v.msg()));
            return;
        }
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}

/// 数据私有化
#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

impl JsonLayer {
    /// 数据入库
    async fn save_db(output: Model) {
        let db = match database::instance() {
            Ok(db) => db,
            Err(err) => {
                println!("{:#?}", err);
                return;
            }
        };
        if let Err(err) = Dao::add(&db, output).await {
            println!("add filed {:#?}", err);
        }
    }

    /// 将异步函数包装成同步函数
    fn sync_task(&self, output: Model) {
        thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(err) => {
                    println!("{:#?}", err);
                    return;
                }
            };
            rt.block_on(async move {
                Self::save_db(output).await;
            });
        });
    }

    /// 获取输出日志
    fn get_output_log(
        &self,
        span_id: Option<u32>,
        metadata: SerializeMetadata<'_>,
        fields: &BTreeMap<String, Value>,
    ) -> Option<Model> {
        let metadata = match serde_json::to_value(metadata) {
            Ok(v) => v,
            Err(err) => {
                println!("get metadata err, err: {:#?}", err);
                return None;
            }
        };
        let mut output: Model = match serde_json::from_value(metadata) {
            Ok(v) => v,
            Err(err) => {
                println!("parse metadata err, err: {:#?}", err);
                return None;
            }
        };

        output.field_data = serde_json::to_string_pretty(&fields).ok();
        output.span_id = span_id;
        output.message = fields.get("message").map(|v| v.to_string());
        output.code = match fields.get("code") {
            Some(v) => v.as_i64().map(|v| v as i32),
            None => None,
        };
        output.code_msg = fields.get("code_msg").map(|v| v.to_string());

        output.user_id = match fields.get("user_id") {
            Some(v) => v.as_i64().map(|v| v as i32),
            None => None,
        };
        output.nickname = fields.get("nickname").map(|v| v.to_string());
        output.created_at = Local::now().naive_local();
        Some(output)
    }

    /// 最大输出日志
    pub fn with_max_level(mut self, level: tracing::Level) -> JsonLayer {
        self.max_level = level;
        self
    }

    /// 过滤数据库日志，防止死循环
    fn filter_db_log(target: &str, fields: &BTreeMap<String, Value>) -> bool {
        // sea_orm::driver::sqlx_sqlite
        // sea_orm::database::db_connection
        // sea_orm::database
        if target.starts_with("sea_orm") {
            return true;
        }
        if fields
            .get("log.target")
            .map_or("", |v| v.as_str().map_or("", |v| v))
            == "sqlx::query"
        {
            return true;
        };
        false
    }
}

/// 输出到数据库中
pub fn layer<S>(config: logger::Options) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let layer = JsonLayer::default().with_max_level(config.level.clone().into());
    Box::new(layer)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    use code::Error;
    use config::logger::Options;
    use tracing::{debug, debug_span, error, info, info_span, trace, warn};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[test]
    fn test_output_json() {
        tracing_subscriber::registry()
            .with(JsonLayer::default())
            .init();

        info!("span outer example");
        let outer_span = info_span!(
            "outer",
            level = 0,
            cc = 5,
            other_field = tracing::field::Empty
        );
        let _outer_entered = outer_span.enter();
        // span 在创建之后，依然要能记录数据。
        outer_span.record("other_field", 7);
        outer_span.record("cc", 10);

        let inner_span = debug_span!("inner", level = 1);
        let _inner_entered = inner_span.enter();

        {
            let inner_span = debug_span!("inner", "xxxxxxxxxx");
            let _inner_entered = inner_span.enter();
            info!("xs");
        }

        info!(a_bool = true, answer = 42, message = "first example");
        info!("second example");
    }

    fn create_err() -> Result<(), Box<dyn std::error::Error + 'static>> {
        Err(Box::new(Error::UnknownError))
    }

    #[test]
    fn test_code_error() {
        tracing_subscriber::registry()
            .with(JsonLayer::default())
            .init();
        info!("second example");
        error!("{}", Error::UnknownError);
        if let Err(err) = create_err() {
            error!(err);
        }
    }

    #[tokio::test]
    async fn test_db() {
        config::init("../../config.toml").expect("初始化配置失败");
        let db_url = "sqlite://../../data.dat".to_string();
        println!("conf: {db_url}");
        let _ = database::init(db_url.clone(), db_url)
            .await
            .expect("初始化数据库失败");

        let db_conf = Options {
            remote_address: "".to_owned(),
            level: config::logger::Level::WARN,
            enable: false,
            ..Options::default()
        };

        tracing_subscriber::registry().with(layer(db_conf)).init();
        trace!("trace example");
        debug!("debug example");
        info!("info example");
        warn!("warn example");
        error!("error example");
        error!("{}", Error::UnknownError);
        if let Err(err) = create_err() {
            error!(err);
        }
        thread::sleep(Duration::from_secs(5));
    }
}
