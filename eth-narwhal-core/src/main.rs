mod rpc;
mod p2p;
mod types;
mod bridge;
mod telemetry;
mod runtime;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use crate::runtime::{spawn_runtime_metrics, TaskPools};
use crate::p2p::P2PHandle;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // 2. Metrics exporter (Prometheus on :9898)
    telemetry::init_metrics_exporter();

    // 3. Runtime saturation metrics (Tokio runtime metrics loop)
    spawn_runtime_metrics("core");

    // 4. Logical task pools (network / consensus)
    let pools = TaskPools::new(
        /* network_limit   */ 32,
        /* consensus_limit */ 16,
    );

    // 5. Start internal “P2P” message loop on the NETWORK pool
    let p2p_handle: P2PHandle = p2p::spawn_p2p(pools.clone()).await?;

    // 6. Start JSON-RPC server – runs until process exit or error
    if let Err(e) = rpc::spawn_rpc(p2p_handle, "0.0.0.0:8545").await {
        tracing::error!("RPC server failed: {e:?}");
    }

    Ok(())
}
