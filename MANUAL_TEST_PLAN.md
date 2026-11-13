# Manual acceptance plan

Status: **not executed; database functionality is not implemented**.
Protocol behavior is tested automatically; it is not evidence of database support.

Record date, tester, OS/architecture, Tabularis version, desktop/web transport,
InfluxDB edition/version/image digest, fixture revision and non-secret settings.

1. Start the pinned fixture (P5 TODO), create two databases and seed mixed-type
   measurements, an empty table and enough rows for several result pages.
2. Install the locally built plugin. Verify driver name, default port, token
   masking, hidden unsupported actions and no credential leakage in diagnostics.
3. Test valid connection, incorrect token, forbidden database, unreachable host,
   timeout, valid TLS and invalid CA. Each failure must produce an actionable error.
4. Browse authorized databases/tables/columns. Check timestamps, tags, fields,
   quoted/unicode identifiers and refresh after a schema change.
5. Run SQL in editor and notebooks. Compare scalar/aggregate/empty results to the
   native CLI. Check integers above 2^53, unsigned values, nulls and ns timestamps.
6. Page through ordered results and queries with existing LIMIT/OFFSET; verify
   row ordering, counts and `has_more` without missing/duplicate rows.
7. Try writes, DDL and mixed read/write scripts; verify enforced protection for
   ordinary execution and read-only RPC paths, including CTE/comment variants.
8. Cancel an active query, then run another; test two connections concurrently,
   close Tabularis and close plugin stdin. Check for orphan processes and workers.
9. Repeat the relevant cases in desktop and web transports.
10. Inspect each release ZIP: executable, .tabularium, license and assets at root;
    verify SHA-256, manifest version, native execution and driver discovery.

Record PASS/FAIL and evidence for each case. Do not mark a milestone complete
without real results; attach screenshots only after removing secrets.
