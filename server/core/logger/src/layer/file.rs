//! 输出到文件
#![allow(unused)]

use crate::config::FileOptions;
use crate::utils::time::local_time;

use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// 同步输出到文件中
/// 每天时轮换的文件追加器
pub fn blocking_layer<S>(config: &FileOptions) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    S: for<'a> LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let file_appender = rolling::daily(config.filepath.clone(), config.filename.clone());
    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        .with_writer(file_appender.with_max_level(config.level.clone().into()));
    Box::new(layer)
}

/// 非阻塞日志输出到文件中
/// 每天时轮换的文件追加器
pub fn non_blocking_layer<S>(
    config: &FileOptions,
) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let file_appender = rolling::daily(config.filepath.clone(), config.filename.clone());

    // 非阻塞
    let (non_blocking, guard) = non_blocking(file_appender);

    let layer = fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(timer)
        .with_writer(non_blocking.with_max_level(config.level.clone().into()))
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
    fn test_blocking_layer() {
        let conf = FileOptions {
            level: config::Level::Debug,
            enable: true,
            ..FileOptions::default()
        };

        let layer = blocking_layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_global_default(subscriber).expect("注册全局日志订阅器失败");

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[test]
    fn test_non_blocking_layer() {
        let conf = FileOptions {
            level: config::Level::Debug,
            enable: true,
            ..FileOptions::default()
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
