//! 数据库日志
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::config::DbOptions;
use crate::dao::Dao;

use database::DatabaseConnection;
use database::Pool;
use entity::log::system::Model;

use chrono::Local;
use serde_json::Value;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tracing::{metadata::LevelFilter, span, Event, Metadata, Subscriber};
use tracing_error::SpanTraceStatus;
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// josn 解析器
#[derive(Debug)]
pub struct JsonLayer {
    name: String,
    max_level: LevelFilter,
    /// 通道发送者, 可以有多个发送者
    tx: Sender<Model>,
    rx: Arc<Mutex<Receiver<Model>>>,
}

impl<S> Layer<S> for JsonLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    /// 用于判断是否启用该层, 判断是否启用某个级别的 span
    fn enabled(&self, _metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        true
    }

    /// 用于处理每次创建 span 时，指定了 follows from 关系的事件，也就是每次调用 span! 宏或其简写形式时，
    /// 传入了 opentracing.followsFrom(span) 参数时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID、follows from 的 span 的 ID 和上下文，
    /// 这些信息可以用来记录或过滤 span，或者执行一些初始化工作。
    // fn on_follows_from(&self, _span: &span::Id, _follows: &span::Id, _ctx: Context<'_, S>) {
    // }

    /// 用于处理每个新创建的 span，也就是每次调用 span! 宏或其简写形式时触发的事件。
    /// 在这个方法中，您可以获取 span 的属性和 ID，这些属性是在创建 span 时指定的，
    /// 例如名称、级别、目标、字段等。
    /// 您可以使用这些信息来记录或过滤 span，或者将它们存储在 span 的扩展中以供后续使用。
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: Context<'_, S>,
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
        let span_id = Some(id.into_u64());
        let metadata = span.metadata();
        let parent_id = span.parent().map(|v| v.id().into_u64());

        let output = self.get_output_log(parent_id, span_id, metadata, &fields, "new_span");

        self.send_data(output);
    }

    /// 事件用于处理每次记录 span 的值，也就是每次调用 record! 宏或其简写形式时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和值，这些值是在记录 span 时指定的，
    /// 例如字段或表达式的结果。您可以使用这些信息来更新或过滤 span，
    /// 或者将它们存储在 span 的扩展中以供后续使用。
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
        let span_id = Some(id.into_u64());
        let metadata = span.metadata();
        let parent_id = span.parent().map(|v| v.id().into_u64());

        let output = self.get_output_log(parent_id, span_id, metadata, fields, "record");

        self.send_data(output);
    }

    /// 用于判断是否启用某个事件
    #[inline]
    fn event_enabled(&self, _event: &Event<'_>, _ctx: Context<'_, S>) -> bool {
        true
    }

    /// 用于处理每个日志事件，也就是每次调用 event! 宏或其简写形式时触发的事件。
    /// 在这个方法中，您可以获取事件的元数据和字段，这些元数据和字段是在创建事件时指定的，
    /// 例如级别、目标、消息等。
    /// 您可以使用这些信息来记录或过滤事件，或者将它们存储在事件的扩展中以供后续使用。
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        // Covert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // 输出日志
        let metadata = event.metadata();
        let parent_id = event.parent().map(|v| v.into_u64());

        let output = self.get_output_log(parent_id, None, metadata, &fields, "event");

        self.send_data(output);
    }

    /// 用于处理每次进入 span 的事件，也就是每次调用 span::Span::enter 方法
    /// 或者使用 span::Span 类型的 enter 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些初始化工作。
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        // 获取内部 span 数据的引用
        let span = ctx.span(id).unwrap();

        // 获取数据的可变引用，该数据是在 on_new_span 中创建的
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let fields: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // 输出日志
        let span_id = Some(id.into_u64());
        let metadata = span.metadata();
        let parent_id = span.parent().map(|v| v.id().into_u64());

        // 日志级别过滤
        if self.filter_level(metadata.level()) {
            return;
        }

        let output = self.get_output_log(parent_id, span_id, metadata, fields, "enter");

        self.send_data(output);
    }

    /// 用于处理每个关闭 span 的事件，也就是每次调用 span::Span::close 方法
    /// 或者使用 span::Span 类型的 drop 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些清理工作。
    // fn on_close(&self, _id: span::Id, _ctx: Context<'_, S>) {}

    /// 用于处理每次退出 span 的事件，也就是每次调用 span::Span::exit 方法
    /// 或者使用 span::Entered 类型的 drop 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些清理工作。
    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        // 获取内部 span 数据的引用
        let span = ctx.span(id).unwrap();

        // 获取数据的可变引用，该数据是在 on_new_span 中创建的
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage =
            extensions_mut.get_mut::<CustomFieldStorage>().unwrap();
        let fields: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // 输出日志
        let span_id = Some(id.into_u64());
        let metadata = span.metadata();
        let parent_id = span.parent().map(|v| v.id().into_u64());

        // 日志级别过滤
        if self.filter_level(metadata.level()) {
            return;
        }

        let output = self.get_output_log(parent_id, span_id, metadata, fields, "exit");
        self.send_data(output);
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
    /// 创建对象
    pub fn new(name: String) -> Self {
        let (tx, rx) = mpsc::channel::<Model>(1000);
        JsonLayer {
            name,
            max_level: tracing::Level::WARN.into(),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    /// 最大输出日志
    pub fn with_max_level(mut self, level: tracing::Level) -> Self {
        self.max_level = level.into();
        self
    }

    /// 过滤日志级别
    fn filter_level(&self, level: &tracing::Level) -> bool {
        self.max_level.lt(level)
    }
    /// 过滤target日志数据
    fn filter_target(&self, target: &str) -> bool {
        if target == "sqlx::query"
            || target == "sea_orm::driver::sqlx_sqlite"
            || target == "sea_orm::driver::sqlx_mysql"
            || target == "sea_orm::database::db_connection"
        // || target == "actix_server::worker"
        {
            return true;
        }
        false
    }

    /// 获取输出日志
    fn get_output_log(
        &self,
        parent_span_id: Option<u64>,
        span_id: Option<u64>,
        metadata: &Metadata,
        fields: &BTreeMap<String, Value>,
        kind: &str,
    ) -> Option<Model> {
        // 日志级别过滤
        if self.filter_level(metadata.level()) {
            return None;
        }
        // 过滤target日志数据
        if self.filter_target(metadata.target()) {
            return None;
        }

        let field_list = metadata
            .fields()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let fields_str = serde_json::to_string(&field_list).map_or("".to_string(), |v| v);

        // 获取当前 span 的 backtrace
        let mut stack = None;
        let backtrace = tracing_error::SpanTrace::capture();
        if backtrace.status() == SpanTraceStatus::EMPTY {
            stack = Some(backtrace.to_string());
        }

        let output = Model {
            // user_id: todo!(),
            // nickname: todo!(),
            parent_span_id: parent_span_id.map(|v| v as u32),
            span_id: span_id.map(|v| v as u32),
            name: self.name.clone(),
            module_path: metadata.module_path().map(|v| v.to_string()),
            target: metadata.target().to_string(),
            file: metadata.file().map(|v| v.to_string()),
            line: metadata.line(),
            level: metadata.level().to_string(),
            is_event: metadata.is_event(),
            is_span: metadata.is_span(),
            kind: kind.to_string(),
            fields: Some(fields_str),
            field_data: serde_json::to_string(&fields).ok(),
            message: fields.get("message").map(|v| v.to_string()),
            stack,
            // code: todo!(),
            // code_msg: todo!(),
            created_at: Some(Local::now().naive_local()),
            ..Default::default()
        };

        // output.code = match fields.get("code") {
        //     Some(v) => v.as_i64().map(|v| v as i32),
        //     None => None,
        // };
        // output.code_msg = fields.get("code_msg").map(|v| v.to_string());

        // output.user_id = match fields.get("user_id") {
        //     Some(v) => v.as_i64().map(|v| v as i32),
        //     None => None,
        // };
        // output.nickname = fields.get("nickname").map(|v| v.to_string());
        // output.created_at = Local::now().naive_local();
        Some(output)
    }

    /// 发送日志数据到通道
    fn send_data(&self, output: Option<Model>) {
        let output = match output {
            Some(v) => v,
            None => return,
        };
        let tx = self.tx.clone();
        if tx.is_closed() {
            return;
        }
        tokio::spawn(async move {
            if let Err(err) = tx.send(output).await {
                println!("receiver closed, err: {:#?}", err);
            }
        });
    }

    /// 循环接收数据入库
    pub fn loop_data(self, address: String) -> Self {
        let rx = self.rx.clone();

        tokio::spawn(async move {
            let wdb = Pool::connect(address).await.expect("初始化数据库失败");
            let db = Pool {
                rdb: DatabaseConnection::Disconnected,
                wdb,
            };
            let dao = Dao::new(&db);
            let mut rx = rx.lock().await;

            while let Some(output) = rx.recv().await {
                if let Err(err) = dao.add(output.clone()).await {
                    println!("log add filed, data: {:?} \nerr: {:?}", output, err);
                }
            }
        });

        self
    }
}

/// 输出到数据库中
pub fn layer<S>(config: &DbOptions) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    let layer = JsonLayer::new(config.log_name.clone())
        .with_max_level(config.level.clone().into())
        .loop_data(config.address.clone());
    Box::new(layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use code::Error;
    use config::logger::DbOptions;

    use once_cell::sync::Lazy;
    use tracing::{debug, debug_span, error, event, info, info_span, trace, warn, Level};
    use tracing_subscriber::layer::SubscriberExt;

    static INIT: Lazy<bool> = Lazy::new(|| {
        let conf = DbOptions {
            address: "sqlite://../../data.dat".to_string(),
            level: config::logger::Level::DEBUG,
            enable: true,
            ..Default::default()
        };

        let layer = layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_global_default(subscriber).expect("注册全局日志订阅器失败");
        true
    });

    fn setup() {
        let _ = INIT;
    }

    #[tokio::test]
    async fn test_log() {
        setup();

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[tokio::test]
    async fn test_event() {
        setup();

        let error = "a bad error";
        event!(Level::ERROR, %error, "Received error");
    }

    #[tokio::test]
    async fn test_outer_record() {
        setup();

        // info!("span outer example");
        let outer_span = info_span!(
            "outer",
            level = 0,
            cc = 5,
            other_field = tracing::field::Empty
        );
        let _outer_entered = outer_span.enter();
        // span 在创建之后，依然要能记录数据。
        // 此时不触发事件
        outer_span.record("other_field", 7);
        outer_span.record("cc", 10);

        // trace!("this is trace");
        // debug!("this is debug");
        // info!("this is info");
        // warn!("this is warn");
        error!("this is error");
    }

    #[tokio::test]
    async fn test_inner_record() {
        setup();

        {
            let inner_span = debug_span!("inner", level = 1, "xxxxxxxxxx");
            let _inner_entered = inner_span.enter();
            trace!("this is inner trace");
            debug!("this is inner debug");
            info!("this is inner info");
            warn!("this is inner warn");
            error!("this is inner error");
        }

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[tokio::test]
    async fn test_inner_record2() {
        setup();

        let inner_span = debug_span!("inner", level = 1);
        let _inner_entered = inner_span.enter();
        {
            // 新建一个事件
            let inner_span = debug_span!("inner2", "xxxxxxxxxx");
            let _inner_entered = inner_span.enter();
            warn!("this is inner warn");
            error!("this is inner error");
        }

        info!(a_bool = true, answer = 42, message = "first example");
        info!("second example");
    }

    /// 模拟产生一个错误
    fn create_err() -> Result<(), Box<dyn std::error::Error + 'static>> {
        Err(Box::new(Error::UnknownError))
    }

    #[tokio::test]
    async fn test_code_error() {
        setup();

        info!("second example");
        error!("{}", Error::UnknownError);
        if let Err(err) = create_err() {
            error!(err);
        }
    }
}
