use std::{env, sync::Arc};

use anyhow::{anyhow, Result};
use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use tracing::{error, info};

#[derive(Clone)]
pub struct SolanaSigner {
    keypair: Arc<Keypair>,
}

impl SolanaSigner {
    pub fn from_env_or_generate() -> Self {
        if let Ok(v) = env::var("SOLANA_BRIDGE_KEYPAIR_BASE64") {
            match base64::decode(&v) {
                Ok(bytes) => {
                    if bytes.len() == 64 {
                        match Keypair::from_bytes(&bytes) {
                            Ok(kp) => {
                                info!("Loaded Solana bridge keypair from env");
                                return Self {
                                    keypair: Arc::new(kp),
                                };
                            }
                            Err(e) => {
                                error!("Failed to parse keypair bytes: {e}; falling back to generated");
                            }
                        }
                    } else {
                        error!(
                            "SOLANA_BRIDGE_KEYPAIR_BASE64 has length {}; expected 64 bytes",
                            bytes.len()
                        );
                    }
                }
                Err(e) => {
                    error!("Failed to base64-decode SOLANA_BRIDGE_KEYPAIR_BASE64: {e}; falling back to generated");
                }
            }
        }

        let mut csprng = OsRng {};
        let kp = Keypair::generate(&mut csprng);
        info!(
            "Generated ephemeral Solana bridge keypair; public key (hex): {}",
            hex::encode(kp.public.to_bytes())
        );

        Self {
            keypair: Arc::new(kp),
        }
    }

    pub fn sign_message_base64(&self, message_b64: &str) -> Result<String> {
        let decoded =
            base64::decode(message_b64).map_err(|e| anyhow!("invalid base64 message: {e}"))?;
        let sig = self.keypair.sign(&decoded);
        Ok(base64::encode(sig.to_bytes()))
    }
}
