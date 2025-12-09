use anyhow::{anyhow, Result};
use crate::types::CrossChainMessage;
use crate::bridge::solana;

pub async fn route_message(msg: CrossChainMessage) -> Result<()> {
    match msg.to_chain.as_str() {
        "solana" => solana::handle_solana(msg).await,
        "aptos" => {
            tracing::warn!("aptos bridge unimplemented");
            Ok(())
        }
        "sui" => {
            tracing::warn!("sui bridge unimplemented");
            Ok(())
        }
        other => Err(anyhow!("Unsupported target chain: {}", other)),
    }
}
