//! 时间工具

use time::{format_description::FormatItem, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;

/// 获取本地时间
pub fn local_time() -> OffsetTime<Vec<FormatItem<'static>>> {
    let time_format = time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
    )
    .expect("format string should be valid!");

    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    // let offset = UtcOffset::current_local_offset().expect("should get local offset!");

    // 本地时间
    OffsetTime::new(offset, time_format)
}
/*
pub fn layer<S, F>(
    local_time: OffsetTime<F>,
    config: logger::Options,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
    F: Formattable + Send + Sync + 'static,
*/
