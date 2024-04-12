use color_eyre::{eyre::eyre, Result};
use config::{ConsoleBunyanConfig, ConsoleConfig, DbConfig, FileConfig};
use database::DbOptions;
use logger::{config, Logger};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::Logger {
        color_eyre: false,
        console: ConsoleConfig {
            level: config::Level::Debug,
            enable: true,
        },
        console_bunyan: ConsoleBunyanConfig {
            level: config::Level::Debug,
            enable: false,
        },
        file: FileConfig {
            level: config::Level::Debug,
            enable: false,
            filepath: "logs".to_owned(),
            filename: "app.log".to_owned(),
        },
        db: DbConfig {
            level: config::Level::Debug,
            enable: false,
            address: "sqlite://./data.dat?mode=rwc".to_owned(),
            log_name: "db_layer".to_owned(),
            options: DbOptions::default(),
        },
    };
    let _guards = Logger::build(&conf).expect("日志初始化失败");

    call_return_err();
    demo1();
    trace!("this is trace");
    debug!("this is debug");
    info!("this is info");
    warn!("this is warn");
    error!("this is error");

    Ok(())
}
