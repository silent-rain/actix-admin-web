//!输出到控制台
use crate::config::ConsoleOptions;
use crate::utils::time::local_time;

use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    layer::{Layer, SubscriberExt},
    registry::LookupSpan,
};

/// 输出到控制台中
pub fn layer<S>(config: &ConsoleOptions) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: SubscriberExt,
    S: for<'a> LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let layer = fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        .with_thread_names(true)
        .log_internal_errors(false)
        .with_writer(std::io::stderr.with_max_level(config.level.clone().into()));
    Box::new(layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config;

    use tracing::{debug, error, info, trace, warn};

    #[test]
    fn test_layer() {
        let conf = ConsoleOptions {
            level: config::Level::Debug,
            enable: true,
        };
        let layer = layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_global_default(subscriber).expect("注册全局日志订阅器失败");

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }
}
