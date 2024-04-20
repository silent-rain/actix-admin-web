//! 数据库日志封装

use std::sync::Arc;

use super::{
    layer::LayerHandler,
    writer::{DbWriter, GuardWriter},
};
use crate::config::DbConfig;

use tracing::Subscriber;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{registry::LookupSpan, Layer};

/// 输出到数据库中
pub fn non_blocking_layer<S>(
    config: DbConfig,
) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
where
    S: Subscriber,
    for<'lookup> S: LookupSpan<'lookup>,
{
    let config1 = config.clone();
    let writer = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let w = DbWriter::new(config1).await;
            Arc::new(w)
        })
    })
    .join()
    .unwrap();

    let layer = LayerHandler::new(writer.clone());

    // 日志循环处理
    writer.loop_data();

    let guard_writer = GuardWriter(writer);
    let (_non_blocking, guard) = non_blocking(guard_writer);
    (Box::new(layer), guard)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    use crate::config;
    use code::Error;

    use tracing::{
        debug, debug_span, error, event, info, info_span, subscriber::DefaultGuard, trace, warn,
        Level,
    };
    use tracing_subscriber::layer::SubscriberExt;

    /// 注册日志订阅器
    fn setup() -> (DefaultGuard, WorkerGuard) {
        let conf = DbConfig {
            // server/core/logger/data.dat
            address: "sqlite://./data.dat?mode=rwc".to_string(),
            level: config::Level::Debug,
            enable: true,
            ..Default::default()
        };

        let (layer, guard) = non_blocking_layer(conf);
        let subscriber = tracing_subscriber::registry().with(layer);
        let trace_guard = tracing::subscriber::set_default(subscriber);

        (trace_guard, guard)
    }

    #[tokio::test]
    async fn test_log() {
        let (_trace_guard, _guard) = setup();

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    #[tokio::test]
    async fn test_event() {
        let (_trace_guard, _guard) = setup();

        let error = "a bad error";
        event!(Level::ERROR, %error, "Received error");
    }

    #[tokio::test]
    async fn test_outer_record() {
        let (_trace_guard, _guard) = setup();

        info!("span outer example");

        let outer_span = info_span!(
            "outer",
            level = 0,
            cc = 5,
            other_field = tracing::field::Empty
        );
        let _outer_entered = outer_span.enter();
        // span 在创建之后，依然要能记录数据。
        // 此时不触发事件
        outer_span.record("other_field", 7);
        outer_span.record("cc", 10);

        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }

    #[tokio::test]
    async fn test_inner_record() {
        let (_trace_guard, _guard) = setup();

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

    #[tokio::test]
    async fn test_inner_record2() {
        let (_trace_guard, _guard) = setup();

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

    /// 模拟产生一个错误
    fn create_err() -> Result<(), Box<dyn std::error::Error + 'static>> {
        Err(Box::new(Error::UnknownError))
    }

    #[tokio::test]
    async fn test_code_error() {
        let (_trace_guard, _guard) = setup();

        info!("second example");
        error!("{}", Error::UnknownError);
        if let Err(err) = create_err() {
            error!(err);
        }
    }
}
