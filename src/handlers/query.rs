use crate::{client, handlers::RpcResult};

/// TODO(P1/P3): real connection probe and protected SQL execution.
/// Never report a successful connection until a real server probe succeeds.
pub fn handle(_method: &str) -> RpcResult {
    Err(client::not_implemented("connection and query execution"))
}
