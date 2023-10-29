//!自定义打印输出日志
use config::logger;

use time::formatting::Formattable;
use tracing::Subscriber;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// 自定义输出
struct CustomWriter;

impl std::io::Write for CustomWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        let text: String = String::from_utf8_lossy(buf).to_string();
        println!("{:#?}", text);
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// 非阻塞日志自定义输出
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
    let (non_blocking_appender, guard) = non_blocking(CustomWriter);
    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(local_time)
        .with_thread_names(true)
        .with_writer(non_blocking_appender.with_max_level(config.level()))
        .boxed();
    (layer, guard)
}
