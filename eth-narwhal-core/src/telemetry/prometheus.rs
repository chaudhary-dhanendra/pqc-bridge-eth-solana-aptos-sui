// eth-narwhal-core/src/telemetry.rs
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing::info;

/// Initialize a Prometheus HTTP exporter on 0.0.0.0:9898
///
/// After this, you can `curl http://localhost:9898/metrics`
/// or point Prometheus at that endpoint.
pub fn init_metrics_exporter() {
    // Make the type explicit so `.parse()` knows what to return.
    let addr: SocketAddr = "0.0.0.0:9898"
        .parse()
        .expect("valid Prometheus listen address");

    let builder = PrometheusBuilder::new().with_http_listener(addr);

    match builder.install() {
        Ok(()) => {
            info!("Prometheus metrics exporter listening on {}", addr);
        }
        Err(e) => {
            // Don't kill the node if metrics fail; just log
            tracing::warn!("Failed to install Prometheus exporter: {e}");
        }
    }
}
