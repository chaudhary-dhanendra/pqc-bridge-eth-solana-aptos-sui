use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::CrossChainMessage;

#[derive(Debug, Deserialize)]
struct BridgeResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BalanceData {
    lamports: u64,
}

/// This mirrors the JSON schema used in the bridge
#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
enum SolanaPayload {
    GetBalance { pubkey: String },
    SendRawTx { tx_base64: String },
    SignMessage { message_base64: String },
}

pub struct SolanaBridgeClient {
    http: Client,
    base_url: String,
}

impl SolanaBridgeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn get_balance(&self, pubkey: &str) -> Result<u64> {
        let url = format!("{}/solana/balance/{}", self.base_url, pubkey);
        let resp = self.http.get(&url).send().await?;
        let body: BridgeResponse<BalanceData> = resp.json().await?;

        if !body.success {
            return Err(anyhow!(body.error.unwrap_or("unknown solana error".into())));
        }
        Ok(body.data.unwrap().lamports)
    }

    pub async fn send_transaction(&self, tx_base64: &str) -> Result<String> {
        let url = format!("{}/solana/sendTx", self.base_url);
        let resp = self.http.post(&url).body(tx_base64.to_string()).send().await?;
        let body: BridgeResponse<String> = resp.json().await?;

        if !body.success {
            return Err(anyhow!(body.error.unwrap_or("solana tx failed".into())));
        }
        Ok(body.data.unwrap())
    }

    pub async fn sign_message(&self, msg_base64: &str) -> Result<String> {
        let url = format!("{}/solana/signMessage", self.base_url);
        let req = serde_json::json!({ "message_base64": msg_base64 });
        let resp = self.http.post(&url).json(&req).send().await?;
        let body: BridgeResponse<serde_json::Value> = resp.json().await?;

        if !body.success {
            return Err(anyhow!(body.error.unwrap_or("solana sign failed".into())));
        }

        let sig = body
            .data
            .and_then(|v| v.get("signature_base64").cloned())
            .ok_or_else(|| anyhow!("missing signature_base64 in response"))?;

        Ok(sig.as_str().unwrap_or_default().to_string())
    }
}

/// Convenience API
pub async fn get_balance(pubkey: &str) -> Result<u64> {
    SolanaBridgeClient::new("http://127.0.0.1:8899")
        .get_balance(pubkey)
        .await
}

/// Main handler used by router.rs
pub async fn handle_solana(msg: CrossChainMessage) -> Result<()> {
    let payload: SolanaPayload = serde_json::from_str(&msg.payload)?;

    let client = SolanaBridgeClient::new("http://127.0.0.1:8899");

    match payload {
        SolanaPayload::GetBalance { pubkey } => {
            let lamports = client.get_balance(&pubkey).await?;
            tracing::info!(
                "Cross-chain Solana GetBalance: {} lamports for {}",
                lamports,
                pubkey
            );
            Ok(())
        }
        SolanaPayload::SendRawTx { tx_base64 } => {
            let sig = client.send_transaction(&tx_base64).await?;
            tracing::info!("Cross-chain Solana SendRawTx: signature {}", sig);
            Ok(())
        }
        SolanaPayload::SignMessage { message_base64 } => {
            let sig = client.sign_message(&message_base64).await?;
            tracing::info!("Cross-chain Solana SignMessage: signature {}", sig);
            Ok(())
        }
    }
}
