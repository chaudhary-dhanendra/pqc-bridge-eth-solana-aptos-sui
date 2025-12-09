# eth-solana-bridge Architecture

This crate exposes a thin HTTP/JSON API for interacting with a Solana RPC node
and signing messages with an ed25519 keypair.

## Modules

- `main.rs` – process entrypoint, wiring config, state, and HTTP router.
- `app` – application state (`AppState`) containing Solana client + signer.
- `config` – environment-driven configuration (`SOLANA_RPC_URL`, etc.).
- `http` – HTTP-facing DTOs and Axum route handlers.
- `solana` – RPC client and signing helper abstractions.
- `telemetry` – tracing/logging setup.
- `cli` – reserved for future command-line tooling.

The Axum handlers call into `SolanaClient` and `SolanaSigner`, which encapsulate
all RPC wire-format and key-handling concerns.
