//! A composable handler for tracing events.

use std::sync::Arc;

use super::{
    visitor::{Storage, StorageVisitor},
    writer::DbWriter,
};

use tracing::{span, Event, Metadata};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// 日志处理 Layer
pub struct LayerHandler {
    writer: Arc<DbWriter>,
}

impl LayerHandler {
    pub fn new(writer: Arc<DbWriter>) -> Self {
        LayerHandler { writer }
    }
}

impl<S> Layer<S> for LayerHandler
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
        // 获取内部 span 数据的引用
        let span = match ctx.span(id) {
            Some(v) => v,
            None => return,
        };

        // 基于 field 值来构建我们自己的 JSON 对象
        let mut visitor = StorageVisitor::default();
        attrs.record(&mut visitor);

        // 获取扩展，用于存储我们的 span 数据
        let mut ext = span.extensions_mut();
        // 存储至 span
        ext.insert::<Storage>(visitor.storage());

        // 输出日志
        let span_id = Some(id.into_u64());
        let span_pid = span.parent().map(|v| v.id().into_u64());
        let metadata = span.metadata();

        self.writer
            .emit(span_pid, span_id, metadata, visitor.storage(), "new_span");
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
        let span = match ctx.span(id) {
            Some(v) => v,
            None => return,
        };

        // 获取数据的可变引用，该数据是在 on_new_span 中创建的
        let mut ext = span.extensions_mut();
        // 获取自定义存储器
        let storage = match ext.get_mut::<Storage>() {
            Some(v) => v,
            None => return,
        };

        // 使用访问器
        let mut visitor = StorageVisitor::from(storage.clone());
        values.record(&mut visitor);

        // 输出日志
        let span_id = Some(id.into_u64());
        let span_pid = span.parent().map(|v| v.id().into_u64());
        let metadata = span.metadata();

        self.writer
            .emit(span_pid, span_id, metadata, storage.clone(), "record");
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
        // 创建新的访问器
        let mut visitor = StorageVisitor::default();
        event.record(&mut visitor);

        // 输出日志
        let span_pid = event.parent().map(|v| v.into_u64());
        let metadata = event.metadata();

        self.writer
            .emit(span_pid, None, metadata, visitor.storage(), "event");
    }

    /// 用于处理每次进入 span 的事件，也就是每次调用 span::Span::enter 方法
    /// 或者使用 span::Span 类型的 enter 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些初始化工作。
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = match ctx.span(id) {
            Some(v) => v,
            None => return,
        };

        let mut ext = span.extensions_mut();
        let storage = match ext.get_mut::<Storage>() {
            Some(v) => v,
            None => return,
        };

        // 输出日志
        let span_id = Some(id.into_u64());
        let span_pid = span.parent().map(|v| v.id().into_u64());
        let metadata = span.metadata();

        self.writer
            .emit(span_pid, span_id, metadata, storage.clone(), "enter");
    }

    /// 用于处理每个关闭 span 的事件，也就是每次调用 span::Span::close 方法
    /// 或者使用 span::Span 类型的 drop 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些清理工作。
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = match ctx.span(&id) {
            Some(v) => v,
            None => return,
        };

        let mut ext = span.extensions_mut();
        let storage = match ext.get_mut::<Storage>() {
            Some(v) => v,
            None => return,
        };

        // 输出日志
        let span_id = Some(id.into_u64());
        let span_pid = span.parent().map(|v| v.id().into_u64());
        let metadata = span.metadata();

        self.writer
            .emit(span_pid, span_id, metadata, storage.clone(), "close");
    }

    /// 用于处理每次退出 span 的事件，也就是每次调用 span::Span::exit 方法
    /// 或者使用 span::Entered 类型的 drop 方法时触发的事件。
    /// 在这个方法中，您可以获取 span 的 ID 和上下文，这些信息可以用来记录或过滤 span，或者执行一些清理工作。
    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = match ctx.span(id) {
            Some(v) => v,
            None => return,
        };

        let mut ext = span.extensions_mut();
        let storage = match ext.get_mut::<Storage>() {
            Some(v) => v,
            None => return,
        };

        // 输出日志
        let span_id = Some(id.into_u64());
        let span_pid = span.parent().map(|v| v.id().into_u64());
        let metadata = span.metadata();

        self.writer
            .emit(span_pid, span_id, metadata, storage.clone(), "exit");
    }
}
