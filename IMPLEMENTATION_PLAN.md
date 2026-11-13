# InfluxDB implementation plan

## Scope and architecture

Proposed MVP: read-only InfluxDB 3 Core SQL via HTTP `/api/v3/query_sql`.
Keep host compatibility through the Tabularis JSON-RPC contract. InfluxDB 1.x,
2.x/Flux, Cloud/Enterprise, writes, DDL, views, routines, indexes, foreign keys,
visual EXPLAIN, SQL import/export and UI extensions are deferred. Adding them
requires an explicit design and independent compatibility evidence.

Current status: scaffolding only. No milestone below is complete.

## P1 — Connection and HTTP transport

Entry points: `src/client.rs`, `src/handlers/query.rs`.

- Add typed connection parsing for `params.params`; validate host, port, database
  and API token without logging secrets. Handle empty/all/multiple database host
  inputs explicitly; do not pick an arbitrary database from an array.
- Use a maintained Rust HTTP client with Rustls. Bearer token authentication;
  support HTTPS and host-provided TLS fields before enabling `supports_ssl`.
  Allow plain HTTP only in an explicitly documented local-fixture policy.
- Define connect/request deadlines, response-size limits, status mapping and
  redaction. Never forward credentials across redirects to another origin.
- Implement real `test_connection` and `ping` probes that validate authorization
  and selected database access, not merely server health.
- Acceptance: local success, unreachable host, invalid token, forbidden database,
  timeout, invalid certificate and response-body redaction are tested. Failed
  probes must remain JSON-RPC errors. Do not return a placeholder success.

## P2 — Metadata

Entry point: `src/handlers/metadata.rs`.

- Verify the chosen Core version's metadata API/catalog queries before coding.
- Map InfluxDB databases to Tabularis databases and measurements/tables to tables;
  keep `schemas: false` unless actual user-visible namespaces require it.
- Implement `get_databases`, `get_tables`, `get_columns`, `get_schema_snapshot`
  and `get_all_columns_batch` with the exact host result shapes.
- Preserve timestamp/tag/field distinctions, nullable columns and native types.
  Define permission-limited discovery and selected-database fallback behavior.
- Return empty metadata only for a verified absence; auth/network failures must
  remain errors. Decide which unsupported relation metadata can safely be empty
  and document it before changing the current explicit errors.
- Acceptance: two databases, two tables, mixed types, empty table, inaccessible
  database, quoted identifiers, fresh and changed schema; compare to native CLI.

## P3 — Query execution and results

Entry point: `src/handlers/query.rs`; add dedicated result/conversion modules.

- Implement SQL submission and mapping to the current host `QueryResult`, column
  metadata and pagination fields. Keep values positional and column order stable.
- Preserve signed/unsigned 64-bit integers, nanosecond timestamps, NULL, strings,
  booleans and floats without silent precision loss. JSON arbitrary precision is
  enabled, but that alone is not proof of host-side numeric fidelity.
- Define pagination over the API's actual response format; prevent duplicate or
  missing rows with stable ordering and truthful `has_more`. Do not blindly wrap
  arbitrary SQL or strip LIMIT/OFFSET from user queries.
- Establish write protection before enabling any execution: read-only credentials
  or server-enforced read-only execution where supported, plus language-aware
  classification as necessary. Prefix checks are not sufficient protection.
  If Core cannot enforce the desired policy, document the constraint and review
  the design before declaring query support or changing `readonly`.
- Acceptance: scalar and aggregate queries, aliases, empty results, malformed SQL,
  large values, multiple pages, existing LIMIT, CTEs/comments, multiple statements,
  blocked writes and read-only-path failure without fallback.

## P4 — Cancellation and lifecycle

Entry points: `src/main.rs`, `src/rpc.rs`, future HTTP transport.

- Process `cancel` notifications while work is active; current loop is synchronous.
- Bound workers and isolate connection context for multiple saved connections.
- Use server cancellation only if the chosen API supports it; otherwise abort the
  client request and state that remote work may continue. Do not claim query timeout
  support without a verified server-side stop.
- Acceptance: unknown/already-finished IDs are harmless; cancel active query;
  subsequent query succeeds; EOF exits even with work active; no leaked workers.

## P5 — Reproducible fixture and usable release

- Pin an exact InfluxDB 3 Core image version (and digest when practical), document
  the license, bootstrap least-privilege test tokens and seed representative data.
- Add mocked transport tests and a separate opt-in real-server integration suite.
  Do not make ordinary unit tests depend on Docker or external accounts.
- Complete MANUAL_TEST_PLAN.md in both desktop and web transports, record exact
  Tabularis/Core versions and OS, and add screenshots without secrets.
- Review capabilities/data_types against implemented behavior, validate manifest,
  run quality gates and inspect all five platform archives. Check standalone
  manifest, executable permissions, filenames and SHA-256 checksums.
- Publish only when these checks pass. Registry submission is a separate action.

## Deferred decisions

InfluxQL/Flux should have explicit language, API, metadata and connection contracts;
not an undocumented mode switch in the SQL handler. Cloud/Enterprise coverage must
record the tested product and authentication method. Writes via line protocol are
separate from generic SQL CRUD and need their own UX/capability design.

## Contributor notes

Use the [repository rules](https://github.com/TabularisDB/tabularis/tree/main/.rules)
and coordinate on [Discord](https://discord.com/invite/K2hmhfHRSt).
