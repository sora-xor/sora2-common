use jsonrpsee::{
    core::{Error as RpcError, RpcResult as Result},
    proc_macros::rpc,
    types::error::CallError,
};
use std::sync::Arc;

#[rpc(client, server)]
pub trait BeefyLightClientAPI {
    #[method(name = "get")]
    fn get(&self) -> Result<u64> {
        Ok(42)
    }
}