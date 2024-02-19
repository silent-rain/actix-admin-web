//!日志
//! 使用案例：
//!     https://zhuanlan.zhihu.com/p/496028010
//!     https://course.rs/logs/tracing.html
//!     https://rustcc.cn/article?id=66e2a76e-8c65-42f7-a773-66dff1a2a21e
//! 自定义日志输出：
//!     https://github.com/rustlang-cn/Rustt/blob/main/Articles/%5B2022-04-07%5D%20%E5%9C%A8%20Rust%20%E4%B8%AD%E4%BD%BF%E7%94%A8%20tracing%20%E8%87%AA%E5%AE%9A%E4%B9%89%E6%97%A5%E5%BF%97.md
//!     https://course.rs/logs/tracing-logger.html#%E5%8A%9F%E8%83%BD%E9%BD%90%E5%85%A8%E7%9A%84-json-logger
//!     https://github.com/bryanburgers/tracing
//!     https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html
pub mod config;
pub mod dao;
mod layer;
pub mod utils;

use config::Logger;

use color_eyre::Result;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};

/// 初始化默认日志
pub fn init_default_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_level(true)
        .with_line_number(true)
        .init();
}

/// 初始化日志
/// let _guards = init(conf)
/// _guards 需要放在主线程，由 main 方法结束后回收
pub fn init(config: Logger) -> Result<Vec<WorkerGuard>> {
    // 日志过滤器
    // let level_filter = EnvFilter::new(config.level);
    // let level_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let mut layers = Vec::new();
    let mut guards = Vec::new();

    // 输出到控制台中
    if config.console.enable {
        let layer = layer::console::layer(&config.console);
        layers.push(layer);
    }

    // bunyan 日志输出到控制台中
    if config.console_bunyan.enable {
        let layer = layer::console_bunyan::layer(&config.console_bunyan);
        layers.push(layer);
    }

    // 输出到文件中
    if config.file.enable {
        let (file_layer, file_guard) = layer::file::non_blocking_layer(&config.file);
        layers.push(file_layer);
        guards.push(file_guard);
    }

    // 输出到数据库
    if config.db.enable {
        layers.push(Box::new(layer::db::layer(&config.db)));
    }

    // 用于针对各种错误的彩色、一致且格式良好的错误报告。
    // color_eyre::install()?;

    // 日志订阅器
    let subscriber = Registry::default()
        // .with(level_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        // .with(console_layer)
        .with(layers);

    // 注册全局日志订阅器
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(guards)
}

#[cfg(test)]
mod tests {
    use super::*;

    use color_eyre::{eyre::eyre, Result};
    use config::{ConsoleBunyanOptions, ConsoleOptions, DbOptions, FileOptions};
    use tracing::{debug, error, info, instrument, span, trace, warn, Level};

    #[instrument]
    fn return_err() -> Result<()> {
        Err(eyre!("Something went wrong"))
    }

    #[instrument]
    fn call_return_err() {
        info!("going to log error");
        if let Err(err) = return_err() {
            // 推荐大家运行下，看看这里的输出效果
            error!(?err, "error");
        }
    }

    fn demo1() {
        let span = span!(Level::TRACE, "my_span");

        // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
        let _enter = span.enter();

        info!("demo1");
    }

    #[test]
    fn test_default_init() {
        init_default_logger();

        call_return_err();
        demo1();
    }

    #[test]
    fn test_init_subscriber() {
        let conf = Logger {
            console: ConsoleOptions {
                level: config::Level::Debug,
                enable: true,
            },
            console_bunyan: ConsoleBunyanOptions {
                level: config::Level::Debug,
                enable: true,
            },
            file: FileOptions {
                level: config::Level::Debug,
                enable: true,
                ..Default::default()
            },
            db: DbOptions {
                level: config::Level::Debug,
                enable: false,
                ..Default::default()
            },
        };
        let _guards = init(conf).expect("日志初始化失败");

        call_return_err();
        demo1();
        trace!("this is trace");
        debug!("this is debug");
        info!("this is info");
        warn!("this is warn");
        error!("this is error");
    }
}
