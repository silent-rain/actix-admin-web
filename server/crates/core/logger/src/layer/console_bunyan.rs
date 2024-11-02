//! 输出到控制台
//! 该层专门涉及使用Bunyan格式格式化信息。
//! 它依赖于上游的JsonStorageLayer来访问连接到每个跨度的字段。
use crate::config::ConsoleBunyanConfig;

use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, registry::LookupSpan, Layer,
};

/// 输出到控制台中
pub fn layer<S>(config: &ConsoleBunyanConfig) -> Box<dyn Layer<S> + Send + Sync + 'static>
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

    use tracing::{
        debug, debug_span, error, event, info, info_span, subscriber::DefaultGuard, trace, warn,
        Level,
    };

    /// 注册日志订阅器
    fn setup() -> DefaultGuard {
        let conf = ConsoleBunyanConfig {
            level: config::Level::Debug,
            enable: true,
        };
        let layer = layer(&conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        tracing::subscriber::set_default(subscriber)
    }

    /// 常规事件日志输出
    #[test]
    fn test_log() {
        let _guard = setup();

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    // 构造一个新事件
    #[test]
    fn test_event() {
        let _guard = setup();

        // 构造一个新事件
        let error = "a bad error";
        event!(Level::ERROR, %error, "Received error");

        // 在信息级别构建一个事件
        info!("span outer example");
    }

    #[test]
    fn test_outer_record() {
        let _guard = setup();
        // 在信息级别构建一个事件
        info!("span outer example");

        // 在信息级别构建span
        let outer_span = info_span!(
            "outer",
            level = 0,
            cc = 5,
            other_field = tracing::field::Empty
        );
        // 进入此span，返回一个防护装置，该防护装置将在掉落时退出span
        let _outer_entered = outer_span.enter();
        // span 在创建之后，依然要能记录数据。
        outer_span.record("other_field", 7);
        outer_span.record("cc", 10);

        // span内日志
        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");

        // span结束
    }

    #[test]
    fn test_inner_record() {
        let _guard = setup();

        {
            // span开始
            let inner_span = debug_span!("inner", level = 1, "xxxxxxxxxx");
            // 进入此span
            let _inner_entered = inner_span.enter();
            trace!("this is inner trace");
            debug!("this is inner debug");
            info!("this is inner info");
            warn!("this is inner warn");
            error!("this is inner error");
            // span结束
        }

        // 常规事件日志
        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[test]
    fn test_inner_record2() {
        let _guard = setup();

        // 进入 inner span
        let inner_span = debug_span!("inner", level = 1);
        let _inner_entered = inner_span.enter();
        {
            // 进入 inner2 span
            let inner_span = debug_span!("inner2", "xxxxxxxxxx");
            let _inner_entered = inner_span.enter();
            warn!("this is inner warn");
            error!("this is inner error");
            // inner2 span 结束
        }

        info!(a_bool = true, answer = 42, message = "first example");
        info!("second example");
        // inner span 结束
    }
}
