//! Prometheus metrics
use actix_web::{web, Scope};
use actix_web_opentelemetry::PrometheusMetricsHandler;
use opentelemetry::{global, metrics::MetricsError};
use opentelemetry_sdk::{metrics::MeterProvider, trace::TracerProvider};
use prometheus;

/// 注册 Prometheus metrics 路由
pub fn register() -> Scope {
    let registry = init_open_telemetry_and_metrics().expect("failed to init prometheus metrics");
    web::scope("/metrics").route(
        "",
        web::get().to(PrometheusMetricsHandler::new(registry.clone())),
    )
}

fn _init_open_telemetry() {
    // Install an OpenTelemetry trace pipeline.
    // Swap for https://docs.rs/opentelemetry-jaeger or other compatible

    // Configure your tracer provider with your exporter(s)
    let provider = TracerProvider::builder().build();
    global::set_tracer_provider(provider);
}

fn init_open_telemetry_and_metrics() -> Result<prometheus::Registry, MetricsError> {
    // Configure prometheus or your preferred metrics service
    let registry = prometheus::Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()?;

    // set up your meter provider with your exporter(s)
    let provider = MeterProvider::builder().with_reader(exporter).build();
    global::set_meter_provider(provider);

    Ok(registry)
}
