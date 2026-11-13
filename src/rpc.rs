use crate::{error, handlers};
use serde_json::{json, Value};

/// Handle a single JSON-RPC request. Valid notifications never get a response.
pub fn handle_line(line: &str) -> Option<Value> {
    let request: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(_) => return Some(error::response(Value::Null, -32700, "Parse error")),
    };
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let valid_id = id.is_null() || id.is_string() || id.is_number();
    let valid_params = request
        .get("params")
        .is_none_or(|p| p.is_object() || p.is_array());
    if !request.is_object()
        || request.get("jsonrpc") != Some(&json!("2.0"))
        || !request.get("method").is_some_and(Value::is_string)
        || !valid_id
        || !valid_params
    {
        return Some(error::response(
            if valid_id { id } else { Value::Null },
            -32600,
            "Invalid request",
        ));
    }
    // Valid notifications do not receive responses.
    // TODO(P4): cancel active work by params.id once queries become asynchronous.
    request.get("id")?;
    let method = request["method"].as_str().expect("validated method");
    let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
    Some(match handlers::dispatch(method, &params) {
        Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
        Err((code, message)) => error::response(id, code, &message),
    })
}
