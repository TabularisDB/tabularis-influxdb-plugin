#!/usr/bin/env python3
"""Create identical ZIP layouts on Unix and Windows, including dotfiles."""
import pathlib
import sys
import zipfile

root = pathlib.Path(__file__).resolve().parents[1]
target, platform = sys.argv[1:3]
suffix = ".exe" if platform.startswith("win-") else ""
binary = root / "target" / target / "release" / ("influxdb-plugin" + suffix)
output = root / "dist"
output.mkdir(exist_ok=True)
with zipfile.ZipFile(output / f"influxdb-plugin-{platform}.zip", "w", zipfile.ZIP_DEFLATED) as archive:
    for path in [binary, root / ".tabularium", root / "LICENSE"]:
        archive.write(path, path.name)
    for path in sorted((root / "assets").rglob("*")):
        if path.is_file():
            archive.write(path, path.relative_to(root))
