//! Config global: /Horizon/mods/rexy_sorter/config.json
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

pub const MOD_DIR: &str = "/Horizon/mods/rexy_sorter";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortMode {
    #[default]
    Alphabetical,
    Category,
    Custom,
}

impl SortMode {
    pub fn next(self) -> Self {
        match self {
            SortMode::Alphabetical => SortMode::Category,
            SortMode::Category => SortMode::Custom,
            SortMode::Custom => SortMode::Alphabetical,
        }
    }
    pub fn index(self) -> i32 {
        self as i32
    }
    pub fn name(self) -> &'static str {
        match self {
            SortMode::Alphabetical => "Alphabetical",
            SortMode::Category => "Category",
            SortMode::Custom => "Custom",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub mode: SortMode,
    pub keybind: String,
    pub show_button: bool,
    pub auto_refill: bool,
    pub locked_slots: Vec<usize>,
    pub custom_order: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: SortMode::default(),
            keybind: "R".into(),
            show_button: true,
            auto_refill: false,
            locked_slots: Vec::new(),
            custom_order: Vec::new(),
        }
    }
}

fn path() -> PathBuf {
    PathBuf::from(MOD_DIR).join("config.json")
}

impl Config {
    pub fn load() -> Config {
        fs::read_to_string(path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
        fs::create_dir_all(MOD_DIR)?;
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path(), json)
    }

    pub fn toggle_lock(&mut self, slot: usize) -> bool {
        if let Some(p) = self.locked_slots.iter().position(|&s| s == slot) {
            self.locked_slots.remove(p);
            false
        } else {
            self.locked_slots.push(slot);
            true
        }
    }
}
