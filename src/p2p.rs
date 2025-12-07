use crate::{router, types::CrossChainMessage};
use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Handle used by RPC to enqueue cross-chain messages.
pub type P2PHandle = mpsc::Sender<CrossChainMessage>;

/// Spawns a background task that receives messages and routes them.
/// For now this is a local in-process queue (no libp2p transport yet).
pub async fn spawn_p2p() -> Result<P2PHandle> {
    let (tx, mut rx) = mpsc::channel::<CrossChainMessage>(128);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = router::route_message(msg).await {
                error!("Failed to route message: {e:?}");
            }
        }
    });

    info!("P2P message loop started (local channel only, no libp2p yet)");
    Ok(tx)
}
