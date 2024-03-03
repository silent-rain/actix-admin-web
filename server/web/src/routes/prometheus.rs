//! Prometheus metrics
use std::time::Duration;

use actix_web::{web, Scope};
use actix_web_opentelemetry::PrometheusMetricsHandler;
use opentelemetry::{global, metrics::MetricsError, trace::TraceError, KeyValue};
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime, trace::TracerProvider, Resource,
};
use prometheus;

/// 注册 Prometheus metrics 路由
pub fn register() -> Scope {
    let (metrics_handler, _provider) =
        init_open_telemetry_and_metrics().expect("failed to init prometheus metrics");

    web::scope("/metrics").route("", web::get().to(metrics_handler.clone()))
}

fn init_open_telemetry_and_metrics(
) -> Result<(PrometheusMetricsHandler, TracerProvider), MetricsError> {
    // Start a new jaeger trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let _tracer = init_tracer().expect("Failed to initialize tracer.");

    // Start a new prometheus metrics pipeline if --features metrics-prometheus is used
    let registry = prometheus::Registry::new();

    // Create a new trace pipeline that prints to stdout
    let provider = TracerProvider::builder().build();

    // global::set_tracer_provider(provider.clone());

    let metrics_handler = PrometheusMetricsHandler::new(registry);

    // Ensure all spans have been reported
    // global::shutdown_tracer_provider();
    // provider.shutdown()?;

    Ok((metrics_handler, provider))
}

pub fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    let trace_config =
        opentelemetry_sdk::trace::config().with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "tracing-jaeger",
        )]));

    let export_config = ExportConfig {
        // endpoint: "http://localhost:4317".to_string(),
        endpoint: "http://localhost:6831".to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::Grpc,
    };

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_trace_config(trace_config)
        .install_batch(runtime::Tokio)
}
