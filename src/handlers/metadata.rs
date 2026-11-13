use crate::{client, handlers::RpcResult};

/// TODO(P2): database/table discovery and typed columns, scoped to the token.
pub fn handle(_method: &str) -> RpcResult {
    Err(client::not_implemented("metadata discovery"))
}
