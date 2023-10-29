//!输出到文件
use config::logger;

use time::formatting::Formattable;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// 同步输出到文件中
/// 每天时轮换的文件追加器
#[allow(unused)]
pub fn layer<S, F>(
    local_time: OffsetTime<F>,
    config: logger::Options,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    S: for<'a> LookupSpan<'a>,
    F: Formattable + Send + Sync + 'static,
{
    // Shared configuration regardless of where logs are output to.
    let file_appender = rolling::daily(config.filepath.clone(), "app.log");
    // let (non_blocking_appender, _guard) = non_blocking(file_appender); // 异步
    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(local_time)
        .with_writer(file_appender.with_max_level(config.level()));
    Box::new(layer)
}

/// 非阻塞日志输出到文件中
/// 每天时轮换的文件追加器
#[allow(unused)]
pub fn non_blocking_layer<S, F>(
    local_time: OffsetTime<F>,
    config: logger::Options,
) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
    F: Formattable + Send + Sync + 'static,
{
    // Shared configuration regardless of where logs are output to.
    let file_appender = rolling::daily(config.filepath.clone(), "app.log");
    let (non_blocking, guard) = non_blocking(file_appender); // 异步
    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(local_time)
        .with_writer(non_blocking.with_max_level(config.level()))
        .boxed();
    (layer, guard)
}
