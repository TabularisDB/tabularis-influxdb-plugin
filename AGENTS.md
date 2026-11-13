# Contributor instructions

- Read README.md and IMPLEMENTATION_PLAN.md before implementing a milestone.
- English code, comments, documentation and commit messages.
- Keep main.rs thin; split HTTP transport, metadata, query and pure utilities.
- stdout is newline-delimited JSON-RPC only; logs go to stderr. Exit on EOF.
- Never log tokens or echo server/request bodies containing credentials.
- Fail explicitly for unimplemented operations; never return fabricated success.
- Advertise capabilities only after implementation and real-server validation.
- Put nontrivial Rust unit tests in sibling tests.rs files; protocol tests in tests/.
- Run fmt, clippy, tests, release build and manifest validation before committing.
- Keep Cargo.toml, Cargo.lock, .tabularium and release tags consistent.
- Follow upstream rules: https://github.com/TabularisDB/tabularis/tree/main/.rules
