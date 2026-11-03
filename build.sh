#!/usr/bin/env bash
# Build Rexy Sorter -> dist/rexy_sorter.hnmod (zip: librexy_sorter.so + manifest.json + config.json)
set -euo pipefail
cd "$(dirname "$0")"

VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
OUT=dist
STAGE=$(mktemp -d)

cargo ndk -t arm64-v8a build --release
SO=target/aarch64-linux-android/release/librexy_sorter.so
[ -f "$SO" ] || { echo "ERROR: $SO tidak ditemukan" >&2; exit 1; }

cp "$SO" "$STAGE/librexy_sorter.so"
cp assets/config.json "$STAGE/config.json"
cat > "$STAGE/manifest.json" <<JSON
{
  "id": "rexy_sorter",
  "name": "Rexy Sorter",
  "version": "$VERSION",
  "author": "Reyz",
  "entry": "librexy_sorter.so"
}
JSON

mkdir -p "$OUT"
rm -f "$OUT/rexy_sorter.hnmod" "$OUT/rexy_sorter.zip"
(cd "$STAGE" && zip -q -X -9 rexy_sorter.zip librexy_sorter.so manifest.json config.json)
mv "$STAGE/rexy_sorter.zip" "$OUT/rexy_sorter.hnmod"
rm -rf "$STAGE"

unzip -l "$OUT/rexy_sorter.hnmod"
echo "OK -> $OUT/rexy_sorter.hnmod"
