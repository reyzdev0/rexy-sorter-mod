#!/usr/bin/env python3
"""Pack mixins/rexy_sorter into dist/rexy_sorter.hnmod (a zip of mixin sources, like other Horizon mods)."""
import json, pathlib, re, zipfile

ROOT = pathlib.Path(__file__).parent
MOD = ROOT / "mixins" / "rexy_sorter"
OUT = ROOT / "dist" / "rexy_sorter.hnmod"
toml = (MOD / "horizon-mixin.toml").read_text()
version = re.search(r'^version = "([^"]+)"', toml, re.M).group(1)
revision = re.search(r'^horizon_revision = "([^"]+)"', toml, re.M).group(1)

native = {
    "game": "26.3",
    "hooks": 80,
    "horizon": revision,
    "id": "rexy_sorter",
    "mixins": "d9c83d7096b7ea5b83678da2efdfd6161f74997e",
    "schema": 1,
    "version": version,
}
OUT.parent.mkdir(exist_ok=True)
with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED) as z:
    z.writestr("horizon-native.json", json.dumps(native, indent=2, sort_keys=True) + "\n")
    for f in sorted(MOD.iterdir()):
        z.write(f, f"mixins/rexy_sorter/{f.name}")
print("OK ->", OUT)
for n in zipfile.ZipFile(OUT).namelist():
    print(" ", n)
