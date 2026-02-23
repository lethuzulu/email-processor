use metrics::{counter, describe_counter, describe_histogram, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

pub fn init_metrics(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    PrometheusBuilder::new()
        .with_http_listener(addr)
        .install()?;

    // describe metrics
    describe_counter!("http_requests_total", "Total HTTP requests");
    describe_histogram!("http_request_duration_seconds", "HTTP request duration");
    describe_counter!("signatures_validated_total", "Total signatures validated");

    Ok(())
}

pub fn record_request(method: &str, path: &str, status: u16) {
    counter!(
        "http_requests_total",
        "method" => method.to_string(),
        "path" => path.to_string(),
        "status" => status.to_string()
    )
    .increment(1);
}

pub fn record_duration(method: &str, path: &str, duration_secs: f64) {
    histogram!(
        "http_request_duration_seconds",
        "method" => method.to_string(),
        "path" => path.to_string()
    )
    .record(duration_secs);
}

pub fn record_validation(valid: bool) {
    counter!(
        "signatures_validated_total",
        "result" => if valid { "valid" } else { "invalid" }
    )
    .increment(1);
}
