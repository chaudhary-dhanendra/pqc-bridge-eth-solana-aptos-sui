mod rpc;
mod p2p;
mod types;
mod router;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Basic logging setup
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Start the internal “P2P” message loop (channel + router)
    let p2p_handle = p2p::spawn_p2p().await?;

    // Start JSON-RPC server (will run until process exit or error)
    if let Err(e) = rpc::spawn_rpc(p2p_handle, "0.0.0.0:8545").await {
        tracing::error!("RPC server failed: {e:?}");
    }

    Ok(())
}
