# Rexy Sorter (.hnmod)

Inventory sorter ala IPN untuk Horizon (Android). Format .hnmod = zip berisi sumber mixin
(`horizon-native.json` + `mixins/rexy_sorter/*`), bukan `.so`.

- `build.py` -> `dist/rexy_sorter.hnmod`
- `cargo test` menguji `sorter.rs` (logika murni)
- Config: `horizon-mods/.horizon-mod-data/rexy_sorter/config.json`

Status: baris Options > Mods + halaman settings (mode, auto refill, tombol) sudah ada.
Hook inventory (tombol Sort, keybind R, lock slot) menunggu target hook inventory Horizon.
