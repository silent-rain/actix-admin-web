# cron scheduler

//! Time is specified for UTC and not your local timezone. Note that the year may be omitted. If you want for your timezone, append \_tz to the job creation calls (for instance Job::new_async vs Job::new_async_tz).

## 调度格式如下

```text
sec   min   hour   day of month   month   day of week   year
*     *     *      *              *       *             *
```

## 参考文档

- [tokio-cron-scheduler](https://crates.io/crates/tokio-cron-scheduler)
