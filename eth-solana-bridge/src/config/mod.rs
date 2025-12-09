use std::env;

#[derive(Clone, Debug)]
pub struct BridgeConfig {
    pub rpc_url: String,
}

impl BridgeConfig {
    pub fn from_env() -> Self {
        let rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

        Self { rpc_url }
    }
}
