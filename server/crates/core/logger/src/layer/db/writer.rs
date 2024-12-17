//! 日志写入
use std::{ops::Deref, sync::Arc};

use super::visitor::Storage;
use crate::config::DbConfig;
use crate::dao::Dao;

use database::{Pool, PoolTrait};
use entity::log::log_system;

use chrono::Local;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use tracing::{info, Metadata};
use tracing_error::SpanTraceStatus;

pub struct DbWriter {
    config: DbConfig,
    db: Pool,
    dao: Arc<Dao<Pool>>,
    /// 通道发送者, 可以有多个发送者
    tx: Sender<log_system::Model>,
    rx: Arc<Mutex<Receiver<log_system::Model>>>,
}

impl DbWriter {
    pub async fn new(config: DbConfig) -> Self {
        // 初始化数据库
        let db = database::Pool::new(config.address.clone(), config.options.clone())
            .await
            .expect("初始化数据库失败");
        let dao = Dao::new(db.clone());

        let (tx, rx) = mpsc::channel::<log_system::Model>(1000);

        DbWriter {
            config,
            db,
            dao: Arc::new(dao),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    /// 写入日志
    pub async fn _write(&self, data: log_system::Model) {
        if let Err(err) = self.dao.add(data.clone()).await {
            println!("log add filed, data: {:?} \nerr: {:?}", data, err);
        }
    }

    /// 关闭数据库和数据通道
    pub fn close(&self) {
        let tx = self.tx.clone();
        let db = self.db.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                tx.closed().await;
                _ = db.close().await;
                info!("close database log channel");
            })
        })
        .join()
        .unwrap();
    }
}

impl DbWriter {
    /// 过滤日志级别
    fn filter_level(&self, level: &tracing::Level) -> bool {
        let max_level: tracing::Level = self.config.level.clone().into();
        max_level.lt(level)
    }

    /// 过滤target日志数据
    ///
    /// 当日志数据库分离后, 将不会再次产生循环日志，因此可进行选择性的忽略日志
    fn filter_target(&self, target: &str) -> bool {
        if target == "tracing_actix_web::root_span_builder"
            // || target == "sqlx::query"
            || target == "sea_orm::driver::sqlx_sqlite"
            || target == "sea_orm::driver::sqlx_mysql"
            || target == "sea_orm::database::db_connection"
        // || target == "actix_server::worker"
        {
            return true;
        }
        false
    }

    /// 获取输出日志
    fn output_log(
        &self,
        span_pid: Option<u64>,
        span_id: Option<u64>,
        metadata: &Metadata,
        storage: Storage,
        kind: &str,
    ) -> Option<log_system::Model> {
        // 日志级别过滤
        if self.filter_level(metadata.level()) {
            return None;
        }
        // 过滤target日志数据
        if self.filter_target(metadata.target()) {
            return None;
        }

        // 获取当前 span 的 backtrace
        let mut stack = None;
        let backtrace = tracing_error::SpanTrace::capture();
        if backtrace.status() == SpanTraceStatus::EMPTY {
            stack = Some(backtrace.to_string());
        }

        let output = log_system::Model {
            // user_id: todo!(),
            // username: todo!(),
            span_pid: span_pid.map(|v| v as u32),
            span_id: span_id.map(|v| v as u32),
            module_path: metadata.module_path().map(|v| v.to_string()),
            target: metadata.target().to_string(),
            file: metadata.file().map(|v| v.to_string()),
            line: metadata.line(),
            level: metadata.level().to_string(),
            is_event: metadata.is_event(),
            is_span: metadata.is_span(),
            kind: kind.to_string(),
            fields: storage.fileds_to_string(),
            field_data: storage.metadata_to_string(),
            message: Some(storage.message()),
            stack,
            code: storage.code().map(|v| v as i32),
            code_msg: storage.code_msg(),
            created_at: Some(Local::now()),
            ..Default::default()
        };

        Some(output)
    }

    /// 发送日志数据到通道
    pub fn emit(
        &self,
        span_pid: Option<u64>,
        span_id: Option<u64>,
        metadata: &Metadata,
        storage: Storage,
        kind: &str,
    ) {
        let output = match self.output_log(span_pid, span_id, metadata, storage, kind) {
            Some(v) => v,
            None => return,
        };
        if self.tx.is_closed() {
            return;
        }
        let tx = self.tx.clone();

        // 尝试获取当前的 tokio 运行时的句柄。
        // 如果能够获取到句柄，那么我们就可以认为当前是在 tokio 运行时的上下文中。
        if tokio::runtime::Handle::try_current().is_err() {
            return;
        }
        tokio::spawn(async move {
            if let Err(err) = tx.send(output).await {
                println!("receiver closed, err: {:#?}", err);
            }
        });
    }

    /// 循环接收数据入库
    pub fn loop_data(&self) {
        let rx = self.rx.clone();
        let dao = self.dao.clone();
        tokio::spawn(async move {
            let mut rx = rx.lock().await;
            while let Some(output) = rx.recv().await {
                if let Err(err) = dao.add(output.clone()).await {
                    println!("log add filed, data: {:?} \nerr: {:?}", output, err);
                }
            }
        });
    }
}

pub struct GuardWriter(pub Arc<DbWriter>);

impl std::io::Write for GuardWriter {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.close();
        Ok(())
    }
}

impl Deref for GuardWriter {
    type Target = DbWriter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
