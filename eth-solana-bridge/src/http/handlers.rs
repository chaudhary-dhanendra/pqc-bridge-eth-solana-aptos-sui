use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use tracing::error;

use crate::app::AppState;
use crate::http::dto::{
    BalanceData, BridgeResponse, SignRequest, SignResponse, SolanaAction,
};

pub async fn get_balance_handler(
    State(state): State<Arc<AppState>>,
    Path(pubkey): Path<String>,
) -> Json<BridgeResponse<BalanceData>> {
    match state.solana.get_balance(&pubkey).await {
        Ok(lamports) => Json(BridgeResponse::ok(BalanceData { lamports })),
        Err(e) => {
            error!("get_balance_handler error: {e:?}");
            Json(BridgeResponse::err(e.to_string()))
        }
    }
}

pub async fn send_tx_handler(
    State(state): State<Arc<AppState>>,
    body: String,
) -> Json<BridgeResponse<String>> {
    let tx_base64 = body.trim();
    if tx_base64.is_empty() {
        return Json(BridgeResponse::err("empty transaction body"));
    }

    match state.solana.send_raw_tx(tx_base64).await {
        Ok(sig) => Json(BridgeResponse::ok(sig)),
        Err(e) => {
            error!("send_tx_handler error: {e:?}");
            Json(BridgeResponse::err(e.to_string()))
        }
    }
}

pub async fn sign_message_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignRequest>,
) -> Json<BridgeResponse<SignResponse>> {
    match state.signer.sign_message_base64(&req.message_base64) {
        Ok(signature_base64) => Json(BridgeResponse::ok(SignResponse { signature_base64 })),
        Err(e) => Json(BridgeResponse::err(e.to_string())),
    }
}

pub async fn handle_action_handler(
    State(state): State<Arc<AppState>>,
    Json(action): Json<SolanaAction>,
) -> Json<BridgeResponse<serde_json::Value>> {
    match action {
        SolanaAction::GetBalance { pubkey } => match state.solana.get_balance(&pubkey).await {
            Ok(lamports) => Json(BridgeResponse::ok(json!({ "lamports": lamports }))),
            Err(e) => Json(BridgeResponse::err(e.to_string())),
        },
        SolanaAction::SendRawTx { tx_base64 } => match state.solana.send_raw_tx(&tx_base64).await {
            Ok(sig) => Json(BridgeResponse::ok(json!({ "signature": sig }))),
            Err(e) => Json(BridgeResponse::err(e.to_string())),
        },
        SolanaAction::SignMessage { message_base64 } => {
            match state.signer.sign_message_base64(&message_base64) {
                Ok(sig_b64) => {
                    Json(BridgeResponse::ok(json!({ "signature_base64": sig_b64 })))
                }
                Err(e) => Json(BridgeResponse::err(e.to_string())),
            }
        }
    }
}
