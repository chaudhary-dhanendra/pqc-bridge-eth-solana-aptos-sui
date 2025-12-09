use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BridgeResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> BridgeResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

#[derive(Serialize)]
pub struct BalanceData {
    pub lamports: u64,
}

#[derive(Deserialize)]
pub struct SignRequest {
    /// Base64-encoded message to sign
    pub message_base64: String,
}

#[derive(Serialize)]
pub struct SignResponse {
    /// Base64-encoded ed25519 signature
    pub signature_base64: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum SolanaAction {
    GetBalance { pubkey: String },
    SendRawTx { tx_base64: String },
    SignMessage { message_base64: String },
}
