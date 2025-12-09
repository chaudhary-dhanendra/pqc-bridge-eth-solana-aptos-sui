// eth-narwhal-core/src/runtime_metrics.rs

use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

/// Spawn a background task that periodically logs a runtime heartbeat.
///
/// This is intentionally minimal and does **not** depend on `tokio-metrics` or
/// any unstable Tokio APIs, so it compiles cleanly on stable Rust.
///
/// You still have a central place to later add real metrics (e.g. queue
/// lengths, in-flight tasks, etc.) and they will be exported via your
/// Prometheus exporter in `telemetry.rs`.
pub fn spawn_runtime_metrics(runtime_label: &'static str) {
    tokio::spawn(async move {
        loop {
            info!(
                target: "runtime_metrics",
                "runtime '{}' heartbeat: metrics task alive",
                runtime_label
            );

            // TODO: extend with real counters/gauges later.
            sleep(Duration::from_secs(5)).await;
        }
    });
}
