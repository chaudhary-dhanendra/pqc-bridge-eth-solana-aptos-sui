use crate::bridge::route_message;
use crate::runtime::TaskPools;
use crate::types::CrossChainMessage;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Handle used by RPC / other subsystems to enqueue cross-chain messages.
pub type P2PHandle = mpsc::Sender<CrossChainMessage>;

/// Spawns a background task that receives messages and routes them.
/// Currently this is an in-process channel; later you can plug in libp2p here.
pub async fn spawn_p2p(pools: TaskPools) -> Result<P2PHandle> {
    let (tx, mut rx) = mpsc::channel::<CrossChainMessage>(256);

    // Run the router loop on the “network” logical pool.
    pools.spawn_network(async move {
        info!("P2P local channel loop started (no libp2p yet)");

        while let Some(msg) = rx.recv().await {
            if let Err(e) = route_message(msg).await {
                error!("Failed to route message: {e:?}");
            }
        }
    });

    Ok(tx)
}
