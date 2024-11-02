//! json 日志

use std::collections::BTreeMap;

use serde_json::Value;
use tracing_subscriber::Layer;

/// josn 解析器
pub struct JsonLayer;

impl<S> Layer<S> for JsonLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // All of the span context
        let scope = ctx.event_scope(event).unwrap();
        let mut spans = vec![];
        for span in scope.from_root() {
            let extensions = span.extensions();
            let storage = extensions.get::<CustomFieldStorage>().unwrap();
            let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
            spans.push(serde_json::json!({
                "target": span.metadata().target(),
                "name": span.name(),
                "file": format!("{:?}", span.metadata().file()),
                "level": format!("{:?}", span.metadata().level().to_string()),
                // "metadata": span.metadata().as_serde(),
                "fields": field_data,
            }));
        }

        // Covert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // Output the event in JSON
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "file": format!("{:?}", event.metadata().file()),
            "level": format!("{:?}", event.metadata().level().to_string()),
            // "metadata": event.metadata().as_serde(),
            "fields": fields,
            "spans": spans,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

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

        // 使用之前创建的 newtype 包裹下
        let storage = CustomFieldStorage(fields);

        // 获取内部 span 数据的引用
        let span = ctx.span(id).unwrap();
        // 获取扩展，用于存储我们的 span 数据
        let mut extensions = span.extensions_mut();
        // 存储！
        extensions.insert::<CustomFieldStorage>(storage);
    }

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
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // 使用我们的访问器老朋友
        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }
}

/// 访问者模式
/// 记录 fields 字典
struct JsonVisitor<'a>(&'a mut BTreeMap<String, Value>);

impl tracing::field::Visit for JsonVisitor<'_> {
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

#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

#[cfg(test)]
mod tests {
    use super::*;

    use tracing::{debug_span, info, info_span};
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    fn test_output_json() {
        let subscriber = tracing_subscriber::registry().with(JsonLayer);
        let _guard = tracing::subscriber::set_default(subscriber);

        // 不支持常规事件
        // info!("span outer example");

        // 进入 span
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

        info!(a_bool = true, answer = 42, message = "first example");
        info!("second example");
    }
}
