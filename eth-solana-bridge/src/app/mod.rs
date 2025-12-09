use crate::config::BridgeConfig;
use crate::solana::{client::SolanaClient, signer::SolanaSigner};
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub solana: SolanaClient,
    pub signer: SolanaSigner,
}

impl AppState {
    pub fn new(cfg: BridgeConfig) -> Self {
        let http = Client::new();
        let solana = SolanaClient::new(cfg.rpc_url.clone(), http);
        let signer = SolanaSigner::from_env_or_generate();

        Self { solana, signer }
    }
}
