//! 自定义打印输出日志
use crate::config::ConsoleOptions;
use crate::utils::time::local_time;

use tracing::Subscriber;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    registry::LookupSpan,
    Layer,
};

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
pub fn non_blocking_layer<S>(
    config: &ConsoleOptions,
) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let (non_blocking_appender, guard) = non_blocking(CustomWriter);

    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(timer)
        .with_thread_names(true)
        .with_writer(non_blocking_appender.with_max_level(config.level.clone().into()))
        .boxed();
    (layer, guard)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config;

    use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

    #[test]
    fn test_non_blocking_layer() {
        let conf = ConsoleOptions {
            level: config::Level::Debug,
            enable: true,
        };

        let (layer, _guard) = non_blocking_layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_global_default(subscriber).expect("注册全局日志订阅器失败");

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }
}
