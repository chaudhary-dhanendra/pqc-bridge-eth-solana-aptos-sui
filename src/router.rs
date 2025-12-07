use crate::types::CrossChainMessage;
use anyhow::Result;
use tracing::info;

/// Very simple router: just logs the message for now.
/// Later you can plug in Solana / Aptos / Sui handlers here.
pub async fn route_message(msg: CrossChainMessage) -> Result<()> {
    info!(
        "Routing message from={} to={} payload={} ts={}",
        msg.from_chain, msg.to_chain, msg.payload, msg.timestamp
    );

    // TODO: integrate real chain-specific handlers here.
    Ok(())
}
