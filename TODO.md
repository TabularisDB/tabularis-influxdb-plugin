# Implementation checklist

Detailed requirements and acceptance criteria: [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

- [x] Rust/stdio scaffold, conservative manifest, protocol tests and SQL quoting helper.
- [x] CI, draft release packaging, license and contribution documentation.
- [ ] P1: typed connection model, HTTP client, token/TLS handling and real probes.
- [ ] P2: permission-aware databases, tables, columns, snapshots and batch metadata.
- [ ] P3: protected SQL execution, exact values, result metadata and deterministic pagination.
- [ ] P4: cancellation, bounded concurrency, multi-connection isolation and EOF cleanup.
- [ ] P5: pinned local fixture, integration tests and desktop/web manual evidence.
- [ ] Review manifest capabilities and native data types against working handlers.
- [ ] Verify draft release archives on all supported operating systems.
- [ ] Publish first usable release and prepare registry submission after acceptance.

Deferred: InfluxDB 1.x/2.x, InfluxQL/Flux, Cloud/Enterprise verification, writes,
DDL, SQL import/export, EXPLAIN and custom UI extensions.
