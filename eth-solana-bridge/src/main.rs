use anyhow::Result;
use axum::{routing::{get, post}, Router};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

mod app;
mod config;
mod http;
mod solana;
mod telemetry;

use app::AppState;
use http::handlers::{
    get_balance_handler, handle_action_handler, send_tx_handler, sign_message_handler,
};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    let cfg = config::BridgeConfig::from_env();
    info!("Using Solana RPC URL: {}", cfg.rpc_url);

    let state = Arc::new(AppState::new(cfg));

    let app = Router::new()
        .route("/solana/balance/:pubkey", get(get_balance_handler))
        .route("/solana/sendTx", post(send_tx_handler))
        .route("/solana/signMessage", post(sign_message_handler))
        .route("/solana/handle", post(handle_action_handler))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8899".parse().unwrap();
    info!("eth-solana-bridge listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
