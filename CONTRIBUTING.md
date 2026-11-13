# Contributing

Read the [README](README.md), [plan](IMPLEMENTATION_PLAN.md) and [AGENTS.md](AGENTS.md).
Pick one milestone and coordinate in an issue before large changes. Start with P1;
query work depends on the connection contract and metadata work depends on a real
server. Mark unimplemented features honestly in code and docs.

Run the README validation commands before sending a pull request. Add pure-helper
unit tests in sibling test modules and protocol/integration coverage under tests/.
For database changes, include actual results from MANUAL_TEST_PLAN.md and record
the exact supported InfluxDB edition/version. Keep secrets out of all artifacts.

Do not enable manifest flags simply because the server has a similarly named
feature; verify the entire Tabularis workflow. Match current upstream RPC types:
https://github.com/TabularisDB/tabularis/blob/main/src-tauri/src/plugins/driver.rs

Follow the [Tabularis rules](https://github.com/TabularisDB/tabularis/tree/main/.rules)
and coordinate on [Discord](https://discord.com/invite/K2hmhfHRSt).
