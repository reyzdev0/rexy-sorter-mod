// Config: <game folder>/horizon-mods/.horizon-mod-data/rexy_sorter/config.json
use super::rexy_sorter_logic::SortMode;
use std::{fs, path::PathBuf, sync::Mutex};

#[derive(Clone, Debug)]
pub struct Config {
    pub mode: SortMode,
    pub keybind: String,
    pub show_button: bool,
    pub auto_refill: bool,
    pub locked_slots: Vec<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: SortMode::default(),
            keybind: "R".to_owned(),
            show_button: true,
            auto_refill: false,
            locked_slots: Vec::new(),
        }
    }
}

pub fn data_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("horizon-mods")
        .join(".horizon-mod-data")
        .join("rexy_sorter")
}

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

fn read() -> Config {
    let d = Config::default();
    let Ok(text) = fs::read_to_string(data_dir().join("config.json")) else {
        write(&d);
        return d;
    };
    let Ok(j) = serde_json::from_str::<serde_json::Value>(&text) else {
        eprintln!("[rexy_sorter] config.json is not JSON; using defaults");
        return d;
    };
    Config {
        mode: j["mode"].as_str().map(SortMode::from_name).unwrap_or(d.mode),
        keybind: j["keybind"].as_str().unwrap_or(&d.keybind).to_owned(),
        show_button: j["show_button"].as_bool().unwrap_or(d.show_button),
        auto_refill: j["auto_refill"].as_bool().unwrap_or(d.auto_refill),
        locked_slots: j["locked_slots"]
            .as_array()
            .map(|a| a.iter().filter_map(|v| v.as_u64()).map(|v| v as usize).collect())
            .unwrap_or_default(),
    }
}

fn write(c: &Config) {
    let json = serde_json::json!({
        "mode": c.mode.name(),
        "keybind": c.keybind,
        "show_button": c.show_button,
        "auto_refill": c.auto_refill,
        "locked_slots": c.locked_slots,
    });
    let _ = fs::create_dir_all(data_dir());
    let text = serde_json::to_string_pretty(&json).unwrap_or_default();
    let _ = fs::write(data_dir().join("config.json"), text);
}

/// Change the config under the lock and save it.
pub fn update<R>(f: impl FnOnce(&mut Config) -> R) -> R {
    let mut slot = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    let cfg = slot.get_or_insert_with(read);
    let out = f(cfg);
    write(cfg);
    out
}

pub fn get() -> Config {
    let mut slot = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    slot.get_or_insert_with(read).clone()
}

pub fn cycle_mode() -> SortMode {
    update(|c| {
        c.mode = c.mode.next();
        c.mode
    })
}

pub fn toggle_refill() -> bool {
    update(|c| {
        c.auto_refill = !c.auto_refill;
        c.auto_refill
    })
}

pub fn toggle_button() -> bool {
    update(|c| {
        c.show_button = !c.show_button;
        c.show_button
    })
}

/// Lock or unlock a slot. Returns true when it is now locked.
pub fn toggle_lock(slot: usize) -> bool {
    update(|c| match c.locked_slots.iter().position(|&s| s == slot) {
        Some(p) => {
            c.locked_slots.remove(p);
            false
        }
        None => {
            c.locked_slots.push(slot);
            true
        }
    })
}
