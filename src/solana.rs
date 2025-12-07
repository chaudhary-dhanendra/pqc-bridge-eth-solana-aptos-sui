use crate::types::CrossChainMessage;

pub async fn handle_solana(msg: CrossChainMessage) {
    println!("🚀 Executing message on Solana: {:?}", msg);
    // TODO: send tx to Solana RPC
}
