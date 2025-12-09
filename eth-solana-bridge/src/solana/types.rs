use serde::Deserialize;

#[derive(Deserialize)]
pub struct RpcResult<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: i64,
}

#[derive(Deserialize)]
pub struct GetBalanceResult {
    pub value: u64,
}
