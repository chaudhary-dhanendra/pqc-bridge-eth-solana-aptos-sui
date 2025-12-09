use anyhow::Result;
use reqwest::Client;
use serde_json::json;

use crate::solana::types::{GetBalanceResult, RpcResult};

#[derive(Clone)]
pub struct SolanaClient {
    rpc_url: String,
    http: Client,
}

impl SolanaClient {
    pub fn new(rpc_url: String, http: Client) -> Self {
        Self { rpc_url, http }
    }

    pub async fn get_balance(&self, pubkey: &str) -> Result<u64> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [pubkey]
        });

        let resp = self.http.post(&self.rpc_url).json(&body).send().await?;
        let parsed: RpcResult<GetBalanceResult> = resp.json().await?;
        Ok(parsed.result.value)
    }

    pub async fn send_raw_tx(&self, tx_base64: &str) -> Result<String> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [tx_base64]
        });

        let resp = self.http.post(&self.rpc_url).json(&body).send().await?;
        let parsed: RpcResult<String> = resp.json().await?;
        Ok(parsed.result)
    }
}
