use anyhow::Result;
use crate::types::CrossChainMessage;

pub async fn handle_sui(_msg: CrossChainMessage) -> Result<()> {
    tracing::warn!("sui bridge path not implemented yet; dropping message");
    Ok(())
}
