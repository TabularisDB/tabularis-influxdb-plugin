# InfluxDB plugin for Tabularis

[![CI](https://github.com/TabularisDB/tabularis-influxdb-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/TabularisDB/tabularis-influxdb-plugin/actions/workflows/ci.yml)

A Rust driver plugin scaffold for [Tabularis](https://github.com/TabularisDB/tabularis),
modeled after [the Trino plugin](https://github.com/TabularisDB/tabularis-trino-plugin).

> **Development scaffold — not a working InfluxDB driver.** The executable builds,
> handles JSON-RPC over stdio, and returns explicit errors for unfinished database
> operations. No connection test, metadata discovery or query execution is implemented.
> No release or registry publication has been made.

## Target and compatibility

The proposed first usable milestone targets **InfluxDB 3 Core**, read-only SQL over
`/api/v3/query_sql` (default port `8181`). This is an implementation target, not a
compatibility claim. InfluxDB 1.x InfluxQL, InfluxDB 2.x Flux, Enterprise and Cloud
editions require separate compatibility work; do not assume they share this API.

See the [implementation plan](IMPLEMENTATION_PLAN.md), [task checklist](TODO.md)
and [manual test plan](MANUAL_TEST_PLAN.md) for acceptance criteria and release gates.

## What exists today

- Modern `.tabularium` driver manifest with stable ID `influxdb`.
- Modular Rust executable with JSON-RPC validation, notification handling and EOF exit.
- Explicit errors for connection/query/metadata stubs and unsupported writes.
- SQL identifier helper, protocol tests and a real stdio subprocess smoke test.
- CI for formatting, Clippy, tests, release build and manifest validation.
- Tag-triggered **draft** releases for Linux x64/arm64, macOS x64/arm64 and Windows x64.
- Apache-2.0 license, contribution guidance and dependency updates.

The manifest disables schema/view/routine browsing, data editing, table management,
SQL import/export, connection strings and TLS UI. These flags must only be enabled
with working handlers and evidence. `readonly: true` disables mutations; currently
all query paths fail, including `execute_read_only_query`. It is not a claim that a
future arbitrary-query implementation is automatically safe.

## Build and validate

Install stable Rust (including rustfmt and Clippy) and Python 3.11+.

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
python3 -m venv .venv
.venv/bin/python -m pip install -r scripts/requirements.txt
.venv/bin/python scripts/validate_manifest.py
```

On Windows, use `.venv\Scripts\python.exe` for the Python commands.
Manifest validation uses a checked-in snapshot of the registry schema; see
[schemas/README.md](schemas/README.md) for its source and update procedure.

## Try the protocol

```sh
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"test_connection","params":{"params":{"host":"localhost","port":8181,"database":"metrics"}}}' | cargo run --quiet
```

Expected response (object key order may differ):

```json
{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"InfluxDB connection and query execution is not implemented in this scaffold; see IMPLEMENTATION_PLAN.md"}}
```

Requests are newline-delimited JSON-RPC 2.0. Connection parameters are nested in
`params.params`, as in the host protocol. Responses preserve the request ID.
Valid notifications (including `cancel`) produce no response; cancellation does
not cancel work yet. Parse errors return `-32700`, invalid requests `-32600`,
unknown/unsupported methods `-32601`, unfinished operations `-32000` and disabled
writes `-32001`. stdout is reserved for protocol output. Closing stdin exits.

## Local installation for development

Build first, then create `drivers/influxdb/` inside Tabularis's plugins directory
and copy `.tabularium` and `target/release/influxdb-plugin` into it. On Windows,
copy `influxdb-plugin.exe` instead. The executable must retain execute permission.

Plugin roots:

| OS | Plugin root |
|---|---|
| Linux | `~/.local/share/tabularis/plugins/` |
| macOS | `~/Library/Application Support/tabularis/plugins/` |
| Windows | `%APPDATA%\tabularis\plugins\` |

Restart Tabularis to discover the driver. It appears in the driver picker, but
connecting intentionally fails until P1 is implemented. Keep real tokens out of
commands, screenshots, fixtures and issue reports.

## Proposed connection contract (TODO)

| Tabularis field | Intended mapping |
|---|---|
| Host and port | InfluxDB 3 Core endpoint, default port 8181 |
| Database | Selected InfluxDB database; permission-scoped discovery is P2 |
| Password, labeled API token | Bearer token; never an unmasked custom setting |
| Username | Hidden; token authentication does not require it |
| TLS | Implement and verify before exposing SSL controls |

No network request is made by the current scaffold.

## Project layout

```text
.tabularium                 Driver and registry metadata
src/main.rs                 Stdin/stdout loop
src/rpc.rs                  JSON-RPC validation and responses
src/error.rs                Error envelope
src/client.rs               HTTP/authentication work entry point (stub)
src/handlers/metadata.rs    Discovery work entry point (stub)
src/handlers/query.rs       Connection/query work entry point (stub)
src/utils/identifiers.rs    SQL identifier quoting
tests/                     Protocol and subprocess tests
scripts/                   Manifest validation and release packaging
schemas/                   Pinned registry schema
```

Add typed connection/result models, HTTP transport and conversion modules as their
milestones need them; avoid unused placeholder APIs or a large `main.rs`.

## Releases

Do not tag a public usable release until P1–P5 and the manual test plan pass.
Bump both `Cargo.toml` and `.tabularium`, regenerate `Cargo.lock`, then tag `vX.Y.Z`.
The workflow checks version consistency, reruns validation, builds five targets,
packages the executable + `.tabularium` + license + assets at ZIP root, writes
SHA-256 checksums, and creates a **draft** GitHub release. The standalone manifest
is uploaded too (GitHub may rename it to `default.tabularium`). Maintainers inspect
all archives and publish the draft only after release acceptance. The workflow
does not submit the plugin to the registry.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md). Start with P1
and coordinate before changing the target edition or query language. Every new
capability needs handler coverage, tests and a real-server check. Follow the
[Tabularis repository rules](https://github.com/TabularisDB/tabularis/tree/main/.rules).
Join [Discord](https://discord.com/invite/K2hmhfHRSt) for coordination.

## References

- [InfluxDB 3 Core HTTP query API](https://docs.influxdata.com/influxdb3/core/query-data/execute-queries/influxdb-v3-api/)
- [InfluxDB 3 Core authentication](https://docs.influxdata.com/influxdb3/core/api/authentication/)
- [Tabularis plugin guide](https://github.com/TabularisDB/tabularis/blob/main/plugins/PLUGIN_GUIDE.md)

## License

[Apache-2.0](LICENSE). The repository icon is an original generic time-series mark,
not an official InfluxData logo.
