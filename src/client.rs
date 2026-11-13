//! TODO(P1): HTTP client, token authentication, TLS and per-request connection context.
//! Keep secrets out of Debug, diagnostics and JSON-RPC errors. Do not add network
//! dependencies until their API and credential handling are covered by tests.

pub fn not_implemented(feature: &str) -> (i32, String) {
    (
        -32000,
        format!(
            "InfluxDB {feature} is not implemented in this scaffold; see IMPLEMENTATION_PLAN.md"
        ),
    )
}
