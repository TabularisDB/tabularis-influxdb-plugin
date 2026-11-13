use serde_json::Value;
pub mod metadata;
pub mod query;

pub type RpcResult = Result<Value, (i32, String)>;

pub fn dispatch(method: &str, _params: &Value) -> RpcResult {
    match method {
        "test_connection" | "ping" | "execute_query" | "execute_read_only_query" => {
            query::handle(method)
        }
        "get_databases"
        | "get_schemas"
        | "get_tables"
        | "get_columns"
        | "get_schema_snapshot"
        | "get_all_columns_batch" => metadata::handle(method),
        "get_views"
        | "get_routines"
        | "get_foreign_keys"
        | "get_indexes"
        | "get_all_foreign_keys_batch" => Err((
            -32601,
            "Metadata feature is not supported by this scaffold".into(),
        )),
        "insert_record"
        | "update_record"
        | "delete_record"
        | "get_create_table_sql"
        | "get_add_column_sql"
        | "get_alter_column_sql"
        | "get_create_index_sql"
        | "get_create_foreign_key_sql"
        | "drop_index"
        | "drop_foreign_key" => Err((
            -32001,
            "Data editing and DDL are disabled for this read-only scaffold".into(),
        )),
        _ => Err((-32601, "Method not found".into())),
    }
}
