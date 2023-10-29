//!输出到控制台
use config::config::logger;

use time::formatting::Formattable;
use tracing::Subscriber;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

#[allow(unused)]

/// 输出到控制台中
pub fn layer<S, F>(
    local_time: OffsetTime<F>,
    config: logger::Options,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
    F: Formattable + Send + Sync + 'static,
{
    // Shared configuration regardless of where logs are output to.
    let layer = fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(local_time)
        .with_thread_names(true)
        .log_internal_errors(false)
        .with_writer(std::io::stderr.with_max_level(config.level()));
    Box::new(layer)
}
