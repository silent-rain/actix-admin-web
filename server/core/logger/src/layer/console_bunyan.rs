//! 输出到控制台
//! 该层专门涉及使用Bunyan格式格式化信息。
//! 它依赖于上游的JsonStorageLayer来访问连接到每个跨度的字段。
use crate::config::ConsoleBunyanOptions;

use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, registry::LookupSpan, Layer,
};

#[allow(unused)]

/// 输出到控制台中
pub fn layer<S>(config: &ConsoleBunyanOptions) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: SubscriberExt,
    S: for<'a> LookupSpan<'a>,
{
    // Shared configuration regardless of where logs are output to.
    let layer = BunyanFormattingLayer::new(
        "console_bunyan_layer".into(),
        std::io::stdout.with_max_level(config.level.clone().into()),
    );
    Box::new(layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config;

    use tracing::{debug, debug_span, error, event, info, info_span, trace, warn, Level};

    fn setup() {
        let conf = ConsoleBunyanOptions {
            level: config::Level::Debug,
            enable: true,
        };
        let layer = layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_global_default(subscriber).expect("注册全局日志订阅器失败");
    }

    #[test]
    fn test_log() {
        setup();

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[test]
    fn test_event() {
        setup();

        let error = "a bad error";
        event!(Level::ERROR, %error, "Received error");
    }

    #[test]
    fn test_outer_record() {
        setup();

        info!("span outer example");
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

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[test]
    fn test_inner_record() {
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

    #[test]
    fn test_inner_record2() {
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
}
