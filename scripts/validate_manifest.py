#!/usr/bin/env python3
"""Validate the pinned registry contract and release-version consistency."""
import json
import pathlib
import sys
import tomllib
import jsonschema

ROOT = pathlib.Path(__file__).resolve().parents[1]
manifest = json.loads((ROOT / ".tabularium").read_text())
schema = json.loads((ROOT / "schemas/driver-manifest.schema.json").read_text())
validator = jsonschema.validators.validator_for(schema)
validator.check_schema(schema)
validator(schema).validate(manifest)
package = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]
assert manifest["version"] == package["version"], "Cargo and manifest versions differ"
assert manifest["executable"] == package["name"], "Executable and package name differ"
if len(sys.argv) > 1:
    assert sys.argv[1].removeprefix("v") == manifest["version"], "Tag and manifest versions differ"
assert manifest["capabilities"]["readonly"]
assert not manifest["capabilities"]["manage_tables"]
assert not manifest["capabilities"]["table_export"]
assert not manifest["capabilities"]["table_import"]
print("Manifest schema, capability guards and version checks passed")
