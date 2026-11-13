use influxdb_plugin::rpc::handle_line;
use serde_json::json;

#[test]
fn malformed_requests_have_protocol_errors() {
    assert_eq!(handle_line("{").unwrap()["error"]["code"], -32700);
    for input in [
        "[]",
        "null",
        r#"{"jsonrpc":"1.0","method":"ping","id":1}"#,
        r#"{"jsonrpc":"2.0","method":"ping","id":true}"#,
        r#"{"jsonrpc":"2.0","method":"ping","params":null,"id":1}"#,
    ] {
        assert_eq!(handle_line(input).unwrap()["error"]["code"], -32600);
    }
}

#[test]
fn notifications_do_not_emit_responses() {
    assert!(handle_line(r#"{"jsonrpc":"2.0","method":"cancel","params":{"id":1}}"#).is_none());
    assert!(handle_line(r#"{"jsonrpc":"2.0","method":"unknown"}"#).is_none());
}

#[test]
fn scaffold_never_claims_connection_success_or_leaks_credentials() {
    for method in [
        "test_connection",
        "ping",
        "get_databases",
        "get_tables",
        "get_columns",
        "execute_query",
        "execute_read_only_query",
    ] {
        let request = json!({"jsonrpc":"2.0", "id":"request-1", "method":method,
            "params":{"params":{"password":"secret-token"},"query":"private-query"}});
        let response = handle_line(&request.to_string()).unwrap();
        assert_eq!(response["id"], "request-1");
        assert_eq!(response["error"]["code"], -32000);
        assert!(response.get("result").is_none());
        assert!(!response.to_string().contains("secret-token"));
        assert!(!response.to_string().contains("private-query"));
    }
}

#[test]
fn unsupported_methods_and_writes_fail_explicitly() {
    for (method, code) in [
        ("unknown", -32601),
        ("insert_record", -32001),
        ("get_create_table_sql", -32001),
    ] {
        let request = json!({"jsonrpc":"2.0","id":7,"method":method});
        let response = handle_line(&request.to_string()).unwrap();
        assert_eq!(response["id"], 7);
        assert_eq!(response["error"]["code"], code);
    }
}
