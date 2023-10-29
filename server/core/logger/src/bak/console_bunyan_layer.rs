//!输出到控制台
use config::config::logger;

use time::formatting::Formattable;
use tracing::Subscriber;
use tracing_bunyan_formatter::BunyanFormattingLayer;
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
    let layer = BunyanFormattingLayer::new(
        "console_bunyan_layer".into(),
        std::io::stdout.with_max_level(config.level()),
    );
    Box::new(layer)
}
