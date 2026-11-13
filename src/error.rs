use serde_json::{json, Value};

/// Construct an error without echoing request payloads or credentials.
pub fn response(id: Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}
