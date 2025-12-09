use anyhow::Result;
use jsonrpsee::{server::ServerBuilder, RpcModule};
use jsonrpsee::core::RpcResult;
use serde::Deserialize;

use crate::{p2p, types::CrossChainMessage};

pub async fn spawn_rpc(handle: p2p::P2PHandle, addr: &str) -> Result<()> {
    let mut module = RpcModule::new(());

    // Simple health check
    module.register_method("bridge_ping", |_params, _, _| -> RpcResult<&'static str> {
        Ok("pong")
    })?;

    // Send a cross-chain message
    let tx = handle.clone();
    module.register_method("bridge_send_message", move |params, _, _| -> RpcResult<&'static str> {
        #[derive(Deserialize)]
        struct SendParams {
            from_chain: String,
            to_chain: String,
            payload: String,
        }

        let input: SendParams = params.one()?;
        let msg = CrossChainMessage::new(input.from_chain, input.to_chain, input.payload);

        // Fire-and-forget; if the channel is full/closed we just log via tracing inside p2p task.
        let _ = tx.try_send(msg);

        Ok("queued")
    })?;

    // Start HTTP JSON-RPC server on `addr`
    let server = ServerBuilder::default().build(addr).await?;
    let _handle = server.start(module);  // <-- no `?` here

    tracing::info!("JSON-RPC server listening on {addr}");
    Ok(())
}
