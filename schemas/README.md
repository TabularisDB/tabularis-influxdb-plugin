# Manifest schema snapshot

`driver-manifest.schema.json` was downloaded on 2026-11-13 from:
https://registry.tabularis.dev/manifest.schema.json?kind=driver

This is the modern merged registry schema, including registry-only fields such
as kind, engine, icon and color. The upstream runtime `manifest.json` schema is
intentionally narrower and should not be used to validate `.tabularium` unchanged.

For deterministic CI we validate against this checked-in snapshot. Refresh it
from the URL above, review the schema diff and rerun scripts/validate_manifest.py
before publishing. Do not edit generated contract fields by hand.
