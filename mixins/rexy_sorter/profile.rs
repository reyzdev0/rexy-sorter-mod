// Profile per world: .../rexy_sorter/profiles/<world>.json
use super::rexy_sorter_config as config;
use super::rexy_sorter_logic::SortMode;
use std::{fs, path::PathBuf};

fn safe_name(world: &str) -> String {
    let s: String = world
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if s.is_empty() { "default".to_owned() } else { s }
}

fn path(world: &str) -> PathBuf {
    config::data_dir().join("profiles").join(format!("{}.json", safe_name(world)))
}

/// Save the current mode, auto refill and locked slots for this world.
pub fn save(world: &str) {
    let c = config::get();
    let json = serde_json::json!({
        "mode": c.mode.name(),
        "auto_refill": c.auto_refill,
        "locked_slots": c.locked_slots,
    });
    let file = path(world);
    if let Some(dir) = file.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let text = serde_json::to_string_pretty(&json).unwrap_or_default();
    let _ = fs::write(file, text);
}

/// Load this world's profile into the config. Returns false if there is none.
pub fn apply(world: &str) -> bool {
    let Ok(text) = fs::read_to_string(path(world)) else {
        return false;
    };
    let Ok(j) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };
    config::update(|c| {
        if let Some(m) = j["mode"].as_str() {
            c.mode = SortMode::from_name(m);
        }
        c.auto_refill = j["auto_refill"].as_bool().unwrap_or(c.auto_refill);
        if let Some(a) = j["locked_slots"].as_array() {
            c.locked_slots = a.iter().filter_map(|v| v.as_u64()).map(|v| v as usize).collect();
        }
    });
    true
}
