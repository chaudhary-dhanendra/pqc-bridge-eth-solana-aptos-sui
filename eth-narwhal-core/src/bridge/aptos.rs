use anyhow::Result;
use crate::types::CrossChainMessage;

pub async fn handle_aptos(_msg: CrossChainMessage) -> Result<()> {
    tracing::warn!("aptos bridge path not implemented yet; dropping message");
    Ok(())
}
