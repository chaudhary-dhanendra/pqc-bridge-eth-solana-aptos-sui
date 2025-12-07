use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainMessage {
    pub from_chain: String,
    pub to_chain: String,
    pub payload: String,
    pub timestamp: u64,
}

impl CrossChainMessage {
    pub fn new(from_chain: String, to_chain: String, payload: String) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            from_chain,
            to_chain,
            payload,
            timestamp: ts,
        }
    }
}
