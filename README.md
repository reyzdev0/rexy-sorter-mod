# Rexy Sorter (.hnmod)

Mod inventory sorter ala Inventory Profiles Next untuk Horizon (Android), ditulis dengan Rust.

- Mode: Alphabetical, Category, Custom
- Tombol `[Rexy Sort]`, keybind `R`, lock slot, auto refill
- Config: `/Horizon/mods/rexy_sorter/config.json`, profile per world: `.../profiles/<world>.json`

Build lokal: `cargo install cargo-ndk && ./build.sh` -> `dist/rexy_sorter.hnmod`.
Build otomatis: GitHub Actions (`.github/workflows/build.yml`), hasil ada di `dist/` dan di Artifacts.

Export C (ABI): `hn_mod_init`, `hn_mod_on_world`, `hn_mod_on_inventory_open`, `hn_mod_on_tap`,
`hn_mod_on_key`, `hn_mod_button_label`, `hn_mod_sort`, `hn_mod_refill`, `hn_mod_cycle_mode`,
`hn_mod_toggle_lock`, `hn_mod_toggle_refill`, `hn_mod_free`.
